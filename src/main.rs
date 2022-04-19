/**
 * Performance :
 *  photo decode: single core
 *  ocr : single core
 *  
 */

use 
{
    std::path::PathBuf,
    //std::collections::VecDeque,
    std::error::Error,
    explorer_lib::*,
    simple_stopwatch as sw,
    tesseract::*,
};

#[tokio::main]
async fn main() ->Result<(),Box<dyn Error>>
{
    ocr().await?;
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
        let img0 = codecs::encoder::encode_to_avif(&x.0, x.1,x.2, 5.0, 3)?;
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

mod test
{
    fn _test400photos() ->Result<(),Box<dyn Error>>
    {
        let exp0 = explorer_lib::Explorer::new("/run/media/akitsuki/Windows/Users/lstsw/Videos/Captures")?;
        let mut matchs = MatchBy::default(); matchs.extensions=Some(EXT_PHOTOS);
        let vphotos = exp0.match_files(matchs)?.iter().map(|x0|{x0.get_path()}).collect::<Vec<_>>();
        for i0 in vphotos.iter()
        {
            let x = codecs::decoder::decode(i0)?;
            let img0 = codecs::encoder::encode_to_avif(&x.0, x.1,x.2, 30.0, 4)?;
            let mut file0 = PathBuf::from(i0.file_name().unwrap());file0.set_extension("avif");
            std::fs::write(file0, img0)?;
        }
        Ok(())
    }

    use super::*;
    pub fn test_screenshot_texts() ->Result<(),Box<dyn Error>>
    {
        let pb0 = PathBuf::from("/run/media/akitsuki/1CDE887D3796F7AA/code-x/oelab_projects/HCRMI/img/text1.png");
        let x = codecs::decoder::decode(&pb0)?;
        for i0 in 1..=2
        {
            for i1 in 1..=9
            {
                let sw0 = sw::Stopwatch::start_new();
                let img0 = codecs::encoder::encode_to_avif(&x.0, x.1,x.2, (i0*5) as f32, i1 as u8)?;
                let itime = sw0.s();
                std::fs::write(format!("img/compare_speed/1_{}_{}_{}.avif",i0*5,i1,itime),img0)?
            }
        }
        Ok(())
    }    
}

#[test]
fn test_to_csv()->Result<(),Box<dyn Error>>
{
    let exp0 = explorer_lib::Explorer::new("/run/media/akitsuki/1CDE887D3796F7AA/code-x/oelab_projects/HCRMI/img/compare_speed/")?;
    let mut matchs = MatchBy::default(); matchs.extensions=Some(EXT_PHOTOS);
    let vphotos = exp0.match_files(matchs)?.iter().map(|x0|{x0.get_path()}).collect::<Vec<_>>();
    let mut s0 = String::new();
    for i0 in vphotos
    {   
        // let char0 = i0.file_name().unwrap().to_str().unwrap().chars();
        let slice = i0.file_name().unwrap().to_str().unwrap().split('_').collect::<Vec<_>>();
        let line0 = format!("{},{},{}\n",slice[1],slice[2],&slice[3].chars().collect::<String>()[..5]);
        s0.push_str(line0.as_str())
    }
    std::fs::write("test.csv", s0)?;
    Ok(())
}

mod codecs;

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

mod tar
{
    use 
    {
        tokio::fs::File,
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
    async fn snap_compress()->Result<(),Box<dyn Error>>
    {
        Ok(())
    }

}

    // ExtermeSpace=(5f32,3u8);
    // SmartPhoneText=(5f32,5u8),
    // Faster=(5f32,9u8),
    // Desktop=(40f32,3u8),
    // Photography=(80f32,2u8)

