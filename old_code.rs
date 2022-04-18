/*
    
*/

use 
{
    std::fs::File
};
#[test]
fn png()->Result<(),Box<dyn std::error::Error>>
{
    let decoder = png::Decoder::new(File::open("img/1.png")?);
    let mut reader = decoder.read_info()?;
    let mut buf = vec![0; reader.output_buffer_size()];
    let info = reader.next_frame(&mut buf).unwrap();
    // let bytes = &buf[..info.buffer_size()];
    let in_animation = reader.info().frame_control.is_some();
    print!("{}",buf.len()/3/3840);
    Ok(())
}
#[test]
fn png2()->Result<(),Box<dyn std::error::Error>>
{
    let decoder = png::Decoder::new(File::open("img/1.png")?);
    let mut reader = decoder.read_info()?;
    let inf0 = reader.info();//println!("{:?}",inf0);
    let (w,_) = (inf0.width,inf0.height);
    let mut cnt0 = 0;
    let mut cnt1 = 0;
    loop 
    {
        match reader.next_row()?
        {
            None=>{break},
            Some(n)=>{cnt1 = n.data().len()}
        };
        cnt0 +=1
    }
    println!("{}",cnt1 as u32 /w); //calculate channel, (RGB)*W*H
    Ok(())
}

#[test]
fn png_to_avif()->Result<(),Box<dyn std::error::Error>>
{
    let decoder = png::Decoder::new(File::open("img/2.png")?);
    let mut reader = decoder.read_info()?;
    let mut buf = vec![0; reader.output_buffer_size()];
    let info = reader.next_frame(&mut buf).unwrap();
    let bytes = &buf[..info.buffer_size()];

    let mut rgb0 = vec![];
    {
        for i0 in 0..buf.len()/3
        {
            rgb0.push(rgb::RGB::new(buf[3*i0],buf[3*i0+1],buf[3*i0+2],));
        }
    }

    let av1cfg = ravif::Config{quality:30.0,alpha_quality:50.0,speed:5,premultiplied_alpha:false,color_space:ravif::ColorSpace::RGB,threads:16};
    let img = ravif::Img::new(&rgb0[..], 3840, 2160);
    let output = ravif::encode_rgb(img, &av1cfg).unwrap();
    println!("{}",output.1);
    let file0 = "img/2r.avif";
    std::fs::write(file0, output.0)?;
    // avif_serialize::serialize(BufWriter::new(File::create(avif0)?), &output.0[..], None, 3840, 2160, 8)?;

    Ok(())
}

pub trait Decode
{
    fn decode(&self)->Vec<rgb::RGB<u8>>;
    fn ocr(&self)->Vec<String>;
} 

pub struct ProtoType <T>
{
    path:Vec<T>,
    buf:Vec<T>,
    ocr_result:Vec<T>,


}

/**
args :
 */

pub enum CompressMode
{
    Quality,
    Storage,
}

pub mod decoders
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
        jpeg_decoder::{Decoder as JpgDecoder,ImageInfo as JpgImageInfo,PixelFormat as JpgPixelFormat},
        png::{Decoder as PngDecoder,BitDepth as PngBitDepth,ColorType as PngColorType,Info as PngInfo},
    };

    #[derive(Debug)]
    struct DecodeErr (String);
    impl DecodeErr { fn new(msg: &str) -> DecodeErr {Self(msg.to_string())}    }
    impl fmt::Display for DecodeErr 
    {   fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {write!(f,"{}",self.0)}    }
    impl Error for DecodeErr 
    {fn description(&self) -> &str {&self.0}    }


    ///Return [R,G,B]*w*h
    pub fn decode_jpg(path:&Path)->Result<(Vec<u8>,(u32,u32)),Box<dyn Error>>
    {
        {
            /*
            jpeg_decoder support 
                Set maximum buffer size
                Returns raw exif data
                Returns the embeded icc profile if the image contains one
                scale the image during decoding

            unsupport since v0.2.4
                Hierarchical

                分层 JPEG。
                ArithmeticEntropyCoding

                JPEG 使用算术熵编码而不是霍夫曼编码。
                SamplePrecision(u8)

                以位为单位的样本精度。 8 位采样精度是当前无损编码过程中支持的。
                ComponentCount(u8)

                图像中的组件数。 当前支持 1、3 和 4 组件。
                DNL

                图像可以在帧头中指定零高度并使用 DNL（定义数量 Lines) 标记在第一次扫描结束时定义帧中的行数。
                SubsamplingRatio

                下采样率。
                NonIntegerSubsamplingRatio

                不能以整数表示的子采样率。
            */
            /*
                pub struct ImageInfo 
                {
                    pub width: u16,
                    pub height: u16,
                    pub pixel_format: PixelFormat,
                    pub coding_process: CodingProcess,
                }            
                pub enum PixelFormat 
                {
                    L8,
                    L16,
                    RGB24,
                    CMYK32,
                }
                pub enum CodingProcess 
                {
                    DctSequential,顺序离散余弦变换 
                    DctProgressive,渐进离散余弦变换 
                    Lossless,无损 
                }                
            */
        } 
        let frame0;
        let mut decoder0 = JpgDecoder::new(BufReader::new(File::open(path)?));
        let _exif0 = decoder0.exif_data();
        let _icc0 = decoder0.icc_profile();
        frame0 = decoder0.decode()?;        
        let (w,h);
        match decoder0.info()
            {
                Some(JpgImageInfo { width, height, pixel_format, coding_process:_ })=>
                {
                    match pixel_format{JpgPixelFormat::RGB24=>{},_=>{return Err(Box::new(DecodeErr::new("unsupport PixelFormat yet [gray or cmyk]")))}}                    
                    w=width;h=height
                },
                None=>{return Err(Box::new(DecodeErr::new("cannt read img info")))}
            };
        Ok((frame0,(w as u32,h as u32)))
    }
    #[test]
    fn tdjpg()->Result<(),Box<dyn Error>>
    {   
        let p0 = PathBuf::from("/home/akitsuki/MEGAsync/Screenshot_20220416_062359.jpg");
        let _ = decode_jpg(p0.as_ref())?;
        Ok(())
    }
    pub fn decode_png(path:&Path)->Result<(Vec<u8>,(u32,u32)),Box<dyn Error>>
    {   // ? 16bit png
        {
            /*
            pub struct Info<'a> 
            {
                非详尽的结构将来可能会添加其他字段。 因此，不能使用传统的方法在外部 crate 中构造非穷举结构 Struct { .. }句法; 没有通配符就无法匹配 ..; 并且 struct update 语法将不起作用。
                width: u32
                height: u32
                bit_depth: BitDepth
                color_type: ColorType

                颜色如何存储在图像中。
                interlaced: bool
                trns: Option<Cow<'a, [u8]>>

                图片 tRNS块（如果存在）； 包含图像调色板的 alpha 通道，每个条目 1 个字节。
                pixel_dims: Option<PixelDimensions>
                palette: Option<Cow<'a, [u8]>>

                图片 PLTE块（如果存在）； 包含图像调色板的 RGB 通道（按此顺序），每个条目 3 个字节（每个通道 1 个）。
                gama_chunk: Option<ScaledFloat>

                图像的 gAMA 块的内容（如果存在）。 更喜欢 source_gamma还可以从 sRGB 块中获取派生的替换伽玛。
                chrm_chunk: Option<SourceChromaticities>

                图片的内容 cHRM块，如果存在的话。 更喜欢 source_chromaticities还可以从 sRGB 块中获取派生替换。
                frame_control: Option<FrameControl>
                animation_control: Option<AnimationControl>
                compression: Compression
                source_gamma: Option<ScaledFloat>

                源系统的 Gamma。 由双方设定 gAMA以及替换为 sRGB块。
                source_chromaticities: Option<SourceChromaticities>

                源系统的色度。 由双方设定 cHRM以及替换为 sRGB块。
                srgb: Option<SrgbRenderingIntent>
                SRGB 图像的渲染意图。
                该值的存在也表明图像符合 SRGB 颜色空间。
                icc_profile: Option<Cow<'a, [u8]>>

                图像的 ICC 配置文件。
                uncompressed_latin1_text: Vec<TEXtChunk>

                文本域
                compressed_latin1_text: Vec<ZTXtChunk>

                zTXt 字段
                utf8_text: Vec<ITXtChunk>

                iTXt 字段

            }
            */
            /*
            May use png info :
                texts,gama_chunk,
            */
            // pub enum BitDepth {
            //     One,
            //     Two,
            //     Four,
            //     Eight,
            //     Sixteen,
            // }
            // pub enum ColorType {
            //     Grayscale,
            //     Rgb,
            //     Indexed,
            //     GrayscaleAlpha,
            //     Rgba,
            // }
        }
        let mut frame0;
        let mut decoder0 = PngDecoder::new(File::open(path)?).read_info()?;
        frame0 = vec![0; decoder0.output_buffer_size()];
        let _ = decoder0.next_frame(&mut frame0)?;
        let inf0 = decoder0.info();
        match inf0.bit_depth
        {
            PngBitDepth::Eight=>{},_=>{return Err(Box::new(DecodeErr::new("unsupport png bitdepth [8bit only now]")))}
        }
        match inf0.color_type
        {
            PngColorType::Rgb=>{},_=>{return Err(Box::new(DecodeErr::new("unsupport png colortype [RGB only now]")))}
        }
        let (w,h) = (inf0.width,inf0.height);
        Ok((frame0,(w,h)))
    }




    #[test]
    fn image_enc_16b_avif()->Result<(),Box<dyn Error>>
    {
        use {image::*};
        let input = image::open("/home/akitsuki/MEGAsync/16bheif.tif")?;
        match input
        {
            DynamicImage::ImageRgb16(t)=>{println!("rgb16")},
            DynamicImage::ImageRgb8(t)=>{println!("rgb8")},
            DynamicImage::ImageRgba16(t)=>
            {
                println!("rgba16");
                // println!("{}",t.as_bytes().len()/3840/2160/4);
                save_buffer_with_format("/home/akitsuki/MEGAsync/test16bit.png", t.as_bytes(), 3840, 2160, ColorType::Rgba16, ImageFormat::Png)?
                // ("/home/akitsuki/MEGAsync/test16bit.avif", t.as_bytes(), 3840, 2160, color)
            },
            _=>{}
        }
        Ok(())
    }
}