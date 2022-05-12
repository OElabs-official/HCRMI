use super::*;
fn _test400photos() ->Result<(),Box<dyn Error>>
{
    let exp0 = explorer_lib::Explorer::new("/run/media/akitsuki/Windows/Users/lstsw/Videos/Captures")?;
    let mut matchs = MatchBy::default(); matchs.extensions=Some(EXT_PHOTOS);
    let vphotos = exp0.match_files(matchs)?.iter().map(|x0|{x0.get_path()}).collect::<Vec<_>>();
    for i0 in vphotos.iter()
    {
        let x = codecs::decoder::decode(i0)?;
        let img0 = codecs::encoder::encode_to_avif(&x.0, x.1,x.2, 30.0, 4,16)?;
        let mut file0 = PathBuf::from(i0.file_name().unwrap());file0.set_extension("avif");
        std::fs::write(file0, img0)?;
    }
    Ok(())
}


pub fn test_screenshot_texts() ->Result<(),Box<dyn Error>>
{
    let pb0 = PathBuf::from("/run/media/akitsuki/1CDE887D3796F7AA/code-x/oelab_projects/HCRMI/img/text1.png");
    let x = codecs::decoder::decode(&pb0)?;
    for i0 in 1..=2
    {
        for i1 in 1..=9
        {
            let sw0 = sw::Stopwatch::start_new();
            let img0 = codecs::encoder::encode_to_avif(&x.0, x.1,x.2, (i0*5) as f32, i1 as u8,16)?;
            let itime = sw0.s();
            std::fs::write(format!("img/compare_speed/1_{}_{}_{}.avif",i0*5,i1,itime),img0)?
        }
    }
    Ok(())
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
