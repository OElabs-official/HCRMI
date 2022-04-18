pub mod decoder
{
    use 
    {
        image::{GenericImageView,EncodableLayout},
        std::
        {
            path::{Path,PathBuf},
            io::{BufReader,},
            fs::File,
            fmt,
            error::Error
        },
    };

    #[derive(Debug)]
    struct DecodeErr (String);
    impl DecodeErr { fn new(msg: &str) -> DecodeErr {Self(msg.to_string())}    }
    impl fmt::Display for DecodeErr 
    {   fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {write!(f,"{}",self.0)}    }
    impl Error for DecodeErr 
    {fn description(&self) -> &str {&self.0}    }

    /*
     *
     * 
     * support ed by crate image
     *      Format 	Decoding 	Encoding
            PNG 	All supported color types 	Same as decoding
            JPEG 	Baseline and progressive 	Baseline JPEG
            GIF 	Yes 	Yes
            BMP 	Yes 	Rgb8, Rgba8, Gray8, GrayA8
            ICO 	Yes 	Yes
            TIFF 	Baseline(no fax support) + LZW + PackBits 	Rgb8, Rgba8, Gray8
            WebP 	Lossy(Rgb only) + Lossless 	No
            AVIF 	Only 8-bit 	Lossy
            PNM 	PBM, PGM, PPM, standard PAM 	Yes
            DDS 	DXT1, DXT3, DXT5 	No
            TGA 	Yes 	Rgb8, Rgba8, Bgr8, Bgra8, Gray8, GrayA8
            OpenEXR 	Rgb32F, Rgba32F (no dwa compression) 	Rgb32F, Rgba32F (no dwa compression)
            farbfeld 	Yes 	Yes
     */

    fn decode_by_image<P: AsRef<Path>>(path:P)->Result<(Vec<u8>,u32,u32),Box<dyn Error>>
    {
        let input = image::open(path)?;
        let (w,h) = input.dimensions();
        let output = input.into_rgb8().as_bytes().to_owned();
        Ok((output,w,h))
    }
    #[test]
    fn decode_any()->Result<(),Box<dyn Error>>
    {
        let _ = decode_by_image("/home/akitsuki/MEGAsync/test16bit.png")?;
         Ok(())
    }

    fn decode_jp2k(path:&Path){}//todo
    fn decode_heif(path:&Path){}//todo
     
    pub fn decode(path:&Path)->Result<(Vec<u8>,u32,u32),Box<dyn Error>>
    {
        match path.extension().unwrap().to_ascii_lowercase().to_str().unwrap()
        {
            "jp2"=>{decode_jp2k(path);Err(Box::new(DecodeErr::new("jp2 format not support yet")))},
            "heic"|"heif"=>{decode_heif(path);Err(Box::new(DecodeErr::new("heif format not support yet")))},
            "avif"=>{Err(Box::new(DecodeErr::new("avif will not process")))}
            _=>{decode_by_image(path)}
        }
    }
}

pub mod encoder
{
    use 
    {
        std::
        {
            path::{Path,PathBuf},
            io::{BufReader,},
            fs::File,
            fmt,
            error::Error
        },
    };
    pub fn encode_to_avif(buf:&Vec<u8>,w:u32,h:u32,quality:f32,speed:u8)->Result<Vec<u8>,Box<dyn Error>>
    {
        let mut rgb0 = vec![];
        {
            for i0 in 0..buf.len()/3
            {rgb0.push(rgb::RGB::new(buf[3*i0],buf[3*i0+1],buf[3*i0+2],));}
        }
        let av1cfg = ravif::Config{quality,alpha_quality:20.0,speed,premultiplied_alpha:false,color_space:ravif::ColorSpace::RGB,threads:num_cpus::get()};
        let img = ravif::Img::new(&rgb0[..], w as usize, h as usize);
        let (output,_) = ravif::encode_rgb(img, &av1cfg).unwrap();
        Ok(output)
    }
}
