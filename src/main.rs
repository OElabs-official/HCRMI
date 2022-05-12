/**
 * Performance :
 *  photo decode: single core
 *  ocr : single core
 *  
 */
mod test;
mod codecs;
mod benchmark;
use 
{
    std::{path::{PathBuf,Path},error::Error,io::stdin},
    explorer_lib::*,
    simple_stopwatch as sw,
    tesseract::*,
    tokio::{fs::File,io::AsyncSeekExt},
    tokio_tar::{Builder,Header},
};

#[tokio::main]async fn main() ->Result<(),Box<dyn Error>>
{
    benchmark::speed_report().await?;
    // ocr().await?;

    // tar::wrap_dir().await?;
{    // let mut buf0 = String::new();
    // let _ = stdin().read_line(&mut buf0)?;
    // prototype(buf0.trim()).await?;
}
    Ok(())
}

async fn prototype<T:AsRef<Path>>(path:T)->Result<(),Box<dyn Error>>
{// println!("{:?}",path.as_ref());ok
    let (quality,speed,threads) = (80f32,3u8,num_cpus::get_physical()); //to expand

    let exp0 = explorer_lib::Explorer::new(&path)?;
    let mut matchs = MatchBy::default(); matchs.extensions=Some(EXT_PHOTOS);
    let vphotos = exp0.match_files(matchs)?.iter().map(|x0|{x0.get_path()}).collect::<Vec<_>>();
    let mut vthread = vec![];
    let mut ocr_js0 = json::JsonValue::new_array();

    let tar_file = File::create(path.as_ref().clone().file_name().unwrap()).await?;
    let mut tar_buld = Builder::new(tar_file);

    async fn thread_decode_ocr_encode(path : &PathBuf,quality: f32,speed: u8,threads: usize) ->Result<(String,PathBuf,f32),Box<dyn Error>>
    {
        let sw0 = sw::Stopwatch::start_new();
        let (ocr_str,avif_bin,mut avif_name);
        {
            let rgb_frame = codecs::decoder::decode(path)?;
            ocr_str = ocr_from_frame(&rgb_frame.0,  rgb_frame.1 as i32, rgb_frame.2 as i32, 3, rgb_frame.1 as i32 *3, "chi_sim")?
                .split(' ').collect::<String>();
            avif_bin = codecs::encoder::encode_to_avif(&rgb_frame.0, rgb_frame.1,rgb_frame.2, quality, speed,threads)?;
            avif_name = PathBuf::from(path.file_name().unwrap());avif_name.set_extension("avif");
            let mut avif_path = PathBuf::from("tmp");avif_path.push(&avif_name);
            let _ = std::fs::create_dir("tmp");std::fs::write(&avif_path, avif_bin)?
        }
        let time0 = sw0.s();
        Ok((ocr_str,avif_name,time0))// _,_,used time
    }
    for i0 in vphotos.iter()
    {
        let path = i0.clone();
        vthread.push(tokio::spawn(async move
        {//println!("thread start path = {:?}",&path);
            thread_decode_ocr_encode(&path,quality,speed,threads).await.unwrap()//consider some photos cannt be decode
        }))
    }
    for i0 in vthread
    {
        let output = i0.await?;
        let mut jv_ocr;
        {
            jv_ocr = json::JsonValue::new_object();
            jv_ocr.insert(output.1.to_str().unwrap(), output.0)?;
            ocr_js0.push(jv_ocr)?;
        }
    }
    ocr_js0.write(&mut std::fs::File::create("tmp/ocr.json")?)?;
    tar_buld.append_dir_all("", "tmp").await?;
    tar_buld.finish().await?;
    std::fs::remove_dir_all("./tmp")?;

    Ok(())
}

// #[test]
async fn ocr() ->Result<(),Box<dyn Error>>
{
    let test_folder = "/run/media/akitsuki/F4DF95DBA7FE8D79/New Folder/fz/Screenshots/";
    let exp0 = explorer_lib::Explorer::new(test_folder)?;
    let mut matchs = MatchBy::default(); matchs.extensions=Some(EXT_PHOTOS);
    let vphotos = exp0.match_files(matchs)?.iter().map(|x0|{x0.get_path()}).collect::<Vec<_>>();
    let mut vthread = vec![];
    let mut js0 = json::JsonValue::new_array();
    async fn thread_ocr(path : &PathBuf) ->Result<(String,PathBuf),Box<dyn Error>>
    {
        let sw0 = sw::Stopwatch::start_new();
        let x = codecs::decoder::decode(path)?;
        let time0 = sw0.s();
        let sw1 = sw::Stopwatch::start_new();
        let ocr = ocr_from_frame(&x.0,  x.1 as i32, x.2 as i32, 3, x.1 as i32 *3, "chi_sim")?;
        let ocr = ocr.split(' ').collect::<String>();
        let time1 = sw1.s();
        println!("{}\ndecode time:{}sec,\nocr time:{}sec",ocr,time0,time1);    
        let img0 = codecs::encoder::encode_to_avif(&x.0, x.1,x.2, 40.0, 3,16)?;
        let mut file0 = PathBuf::from(path.file_name().unwrap());file0.set_extension("avif");
        std::fs::write(file0.clone(), img0)?;
        Ok((ocr,file0))
    }
    for i0 in vphotos.iter()
    {
        let path = i0.clone();
        vthread.push(tokio::spawn(async move
        {
            println!("thread start path = {:?}",&path);
            thread_ocr(&path).await.unwrap()
        }))

    }
    for i0 in vthread
    {
        let s0 = i0.await?;
        let mut jv0 = json::JsonValue::new_object();
        jv0.insert(s0.1.to_str().unwrap(), s0.0)?;
        js0.push(jv0)?;
    }
    js0.write(&mut std::fs::File::create("ocr.json")?)?;

    // std::fs::write("ocr.json", js0)?;
    Ok(())
}

mod tar
{
    use 
    {
        tokio::{fs::File,io::AsyncSeekExt},
        tokio_tar::Builder,
        std::error::Error,
    };
    #[tokio::test]
    async fn write_tar()->Result<(),Box<dyn Error>>
    {
        let file = File::create("test.tar").await?;
        let mut a = Builder::new(file);
        let inner = "/run/media/akitsuki/1CDE887D3796F7AA/code-x/oelab_projects/HCRMI/img/1.avif";
        a.append_file("1.avif", &mut File::open(inner).await?).await?;
        Ok(())
    }
    #[test]
    fn lz4_compress()->Result<(),Box<dyn Error>> // test from 448k to 351kb
    {
        let target = "ocr.lz4";
        let json = "/run/media/akitsuki/F4DF95DBA7FE8D79/New Folder/fz/ss_avif/ocr.json";
        let jf = json::parse(std::fs::read_to_string(json)?.as_str())?;
        let compressed_file = std::fs::File::create(target)?;
        let mut compressor = lz4_flex::frame::FrameEncoder::new(compressed_file);
        jf.write(&mut compressor)?;
        compressor.finish()?;
        Ok(())
    }
    //test 
    // #[tokio::test]
    pub async fn wrap_dir()->Result<(),Box<dyn Error>>
    {
        let file = File::create("test.hcrmi").await?;
        let mut a = Builder::new(file);
        let exp0 = explorer_lib::Explorer::new("/run/media/akitsuki/F4DF95DBA7FE8D79/New Folder/fz/ss_avif/")?;
        let mut matchs = explorer_lib::MatchBy::default(); matchs.extensions=Some(&[""]);
        let vpath = exp0.match_files(matchs)?.iter().map(|x0|{x0.get_path()}).collect::<Vec<_>>();
        // println!("{}",vpath.len());
        for i0 in vpath
        {
            a.append_file(i0.file_name().unwrap(), &mut File::open(&i0).await?).await?;
        }
        Ok(())
    }
    

}

// ExtermeSpace=(5f32,3u8);
// SmartPhoneText=(5f32,5u8),
// Faster=(5f32,9u8),
// Desktop=(40f32,3u8),
// Photography=(80f32,2u8)

struct ProtoType
{
    inputs : Vec<PathBuf>,
    avifs : Vec<Vec<u8>>,
    ocr_results : Vec<String>,
    configs : json::JsonValue,
    tar : Vec<u8>
}
impl ProtoType
{
    pub fn init(){}
    pub fn input_forder(){}
    pub fn input_flie_paths(){}
}

mod speed_and_quality
{
    //recommend least 32-core with high feq cpu
    pub static EXTERME_SPACE : (f32,u8) = (5f32,1u8);
    pub static EXTERME_BALANCE : (f32,u8) = (30f32,1u8);
    pub static EXTERME_PHOTOGRAPH : (f32,u8) = (80f32,1u8);

    //recommend 16-core cpu desktop
    pub static ULTRA_SPACE : (f32,u8) = (5f32,2u8);
    pub static ULTRA_BALANCE : (f32,u8) = (30f32,2u8);
    pub static ULTRA_PHOTOGRAPH : (f32,u8) = (80f32,2u8);

    //recommend 8-core cpu, such as 12gen ultrabook , amd ryzen3 laptop, or desktop
    pub static BETTER_SPACE : (f32,u8) = (5f32,3u8);
    pub static BETTER_BALANCE : (f32,u8) = (30f32,3u8);
    pub static BETTER_PHOTOGRAPH : (f32,u8) = (80f32,3u8);

    //recommend 4-core cpu, such as 10-11gen ultrabook
    pub static SPACE : (f32,u8) = (5f32,5u8);
    pub static BALANCE : (f32,u8) = (30f32,5u8);
    pub static PHOTOGRAPH : (f32,u8) = (80f32,5u8);

    //recommend 2-core cpu, old pc or low-power platform
    pub static FASTER_SPACE : (f32,u8) = (10f32,9u8);
    pub static FASTER_BALANCE : (f32,u8) = (40f32,9u8);
    pub static FASTER_PHOTOGRAPH : (f32,u8) = (90f32,9u8);    
}