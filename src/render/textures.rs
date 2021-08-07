use glium::texture::Texture2dArray;

static mut TEXTURE_ARRAY: Option<Texture2dArray> = None;
static mut TEXTURE_COUNT: u32 = 0;
static mut TEXTURES: Option<Vec<Texture>> = None;
static IMAGE_SIZE: u32 = 800;

#[derive(Clone)]
pub struct Texture {
    texture_index: usize,
    imgui_id: imgui::TextureId,
}

impl Texture {
    pub fn new(texture_index: usize, imgui_id: imgui::TextureId) -> Self {
        Self {
            texture_index,
            imgui_id
        }
    }

    pub fn get_texture_index(&self) -> usize {
        self.texture_index
    }

    pub fn get_imgui_id(&self) -> imgui::TextureId {
        self.imgui_id
    }
}

pub fn load_texture_array(facade: &impl glium::backend::Facade, textures: &mut imgui::Textures<imgui_glium_renderer::Texture>) {
    unsafe {
        use crate::glium::Surface;

        println!("Initializing textures");

        let texture_paths = load_texture_paths();
        TEXTURE_COUNT = texture_paths.len() as u32;
        TEXTURES = Some(Vec::with_capacity(TEXTURE_COUNT as usize));
        let mut raw_images = Vec::with_capacity(TEXTURE_COUNT as usize);

        println!("Found {} textures", TEXTURE_COUNT);
        
        for dir_entry in texture_paths {
            println!("Loading {}/{}", TEXTURES.as_ref().unwrap().len() + 1, TEXTURE_COUNT);
            // load texture
            let image = image::open(dir_entry.path()).unwrap().to_rgba8();
            
            let image_dimensions = image.dimensions();
            let raw_image = glium::texture::RawImage2d::from_raw_rgba_reversed(&image.into_raw(), image_dimensions);
            
            // shrink/expand image to fit into IMAGE_SIZE texture
            let image = glium::texture::Texture2d::new(facade, raw_image).unwrap();
            
            let small_side = std::cmp::min(image_dimensions.0, image_dimensions.1);
            let target_image = glium::texture::Texture2d::empty(facade, IMAGE_SIZE, IMAGE_SIZE).unwrap();
            
            {
                let image_surface = image.as_surface();
                let target_surface = target_image.as_surface();
                
                image_surface.blit_color(
                    &glium::Rect {
                        left: 0,
                        bottom: 0,
                        width: small_side,
                        height: small_side,
                    },
                    &target_surface,
                    &glium::BlitTarget {
                        left: 0,
                        bottom: 0,
                        width: IMAGE_SIZE as i32,
                        height: IMAGE_SIZE as i32,
                    },
                    glium::uniforms::MagnifySamplerFilter::Linear
                );
            }
            
            let source:glium::texture::RawImage2d<u8> = target_image.read();
            
            let texture_id = textures.insert(imgui_glium_renderer::Texture
                {
                    texture: std::rc::Rc::new(target_image),
                    sampler: Default::default()
                }
            );

            TEXTURES.as_mut().unwrap().push(Texture::new(raw_images.len(), texture_id));

            raw_images.push(source);
        }

        TEXTURE_ARRAY = Some(Texture2dArray::new(
            facade,
            raw_images
        ).unwrap());

        println!("Finished initializing textures");
    }
}

pub fn get_texture_array() -> &'static Texture2dArray {
    unsafe {
        TEXTURE_ARRAY.as_ref().expect("Textures were not initialized")
    }
}

fn is_image(extension: &std::ffi::OsStr) -> bool {
    let path = extension.to_str().unwrap().to_lowercase();

    match path {
        p if p.eq("png") => true,
        p if p.eq("jpg") => true,
        _ => false
    }
}

fn load_texture_paths() -> std::vec::Vec<std::fs::DirEntry> {
    let path = std::path::Path::new("./textures");

    let mut results = Vec::with_capacity(20);

    if path.is_dir() {
        for entry in std::fs::read_dir(path).unwrap() {
            let entry = entry.unwrap();
            let file_name = entry.file_name();
            let extension = std::path::Path::new(&file_name).extension().unwrap();

            if is_image(extension) {
                results.push(entry);
            }
        }
    }

    results
}

pub fn get_texture_count() -> u32 {
    unsafe {
        return TEXTURE_COUNT;
    }
}

pub fn get_texture_ids() -> &'static Vec<Texture> {
    unsafe {
        return TEXTURES.as_ref().unwrap()
    }
}