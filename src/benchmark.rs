/**
 * test photos :  screenshot,photography
 * 
 * quality test :
 *  pc 4k screenshot:
 *      5-95 , +10 per step, speed 3
 *      compare size
 * 
 *  smartphone photography:
 *      5-95 , +10 per step, speed(2,3,5)
 *      compare size, psnr via opencv
 *  
 * speed test:
 *  test 1-2-4-8-16 thread(s) for encode, record the time
 *      ! lock the cpu feq to 3GHz via throttle stop  (windows) 
 *      test speed (2,3,5) encode 32times/speed preset,
 *      test different quality (5,35,55,85,95)
 *      
 */
use 
{
    std::{path::{PathBuf,Path},error::Error,io::stdin},
    super::*,
};
pub async fn _speed_test()->Result<(),Box<dyn Error>>
{
    let mut record = "speed,threads,quality,encode_time\n".to_string();
    let path = "benchmark/speed_test.png";
    let rgb_frame = codecs::decoder::decode(&PathBuf::from(path))?;
    async fn thread_encode(rgb_frame: (Vec<u8>, u32, u32),quality: f32,speed: u8,threads: usize)->Result<(),Box<dyn Error>>
    {
        let _ = codecs::encoder::encode_to_avif(&rgb_frame.0, rgb_frame.1,rgb_frame.2, quality, speed,threads)?;
        Ok(())
    }
    for speed in [2,3,5]
    {
        for threads in [1,2,4,8,16]
        {
            for quality in [5,35,55,85,95]
            {
                println!("start test >>> speed:{},threads:{},quality:{}",speed,threads,quality);
                let sw = sw::Stopwatch::start_new();
                let mut vthread = vec![];                
                for _ in 0..32
                {
                    let irgb_frame = rgb_frame.clone();
                    vthread.push(tokio::spawn(async move
                    {
                        thread_encode(irgb_frame, quality as f32, speed, threads).await.unwrap()
                    }))
                }
                for i0 in vthread
                {
                    let _ = i0.await?;
                }
                let itime = sw.s();
                println!("speed:{},threads:{},quality:{},encode_time:{}",speed,threads,quality,itime);
                record.push_str(format!("{},{},{},{}\n",speed,threads,quality,itime).as_str());
            }
        }        
    }
    std::fs::write("benchmark/result_speed.csv", record)?;

    Ok(())
}

pub async fn speed_report()->Result<(),Box<dyn Error>>
{
    let mut log = String::from("<< HCRMI BENCHMARK >>\n\n");
    let files = 
    ["benchmark/4k_wikipedia_desktop.png",
    "benchmark/2k+_wikipedia_smartphone.png",
    "benchmark/7.7MP_photography_smartphone.jpg"];
    let rgb_frames = files.iter().map(|x|{codecs::decoder::decode(&PathBuf::from(x)).unwrap()}).collect::<Vec<_>>();
    {//ocr speed test
        let sw = sw::Stopwatch::start_new();
        let _  = ocr_from_frame(&rgb_frames[1].0,  rgb_frames[1].1 as i32, rgb_frames[1].2 as i32, 3, rgb_frames[1].1 as i32 *3, "chi_sim")?
        .split(' ').collect::<String>();
        let rpt = format!("ocr use time : +{}sec\n<!>ocr only running in single thread<!>\n\n",sw.s());
        log.push_str(rpt.as_str());
        println!("{}",rpt)
    }
    {//single photo encode
        for speed in [2,3,5]
        {
            for quality in [5,15,35]
            {
                let sw = sw::Stopwatch::start_new();
                let _ = codecs::encoder::encode_to_avif(&rgb_frames[1].0, rgb_frames[1].1,rgb_frames[1].2, quality as f32, speed,num_cpus::get())?;
                let rpt = format!("encode <2k+ smartphone screenshot> quality = {} , speed = {} , use time : +{}sec\n",quality,speed,sw.s());
                log.push_str(rpt.as_str());println!("{}",rpt)
            }
        }
        for speed in [2,3,5]
        {
            for quality in [5,15,35]
            {
                let sw = sw::Stopwatch::start_new();
                let _ = codecs::encoder::encode_to_avif(&rgb_frames[0].0, rgb_frames[0].1,rgb_frames[0].2, quality as f32, speed,num_cpus::get())?;
                let rpt = format!("encode <4k+ desktop screenshot> quality = {} , speed = {} , use time : +{}sec\n",quality,speed,sw.s());
                log.push_str(rpt.as_str());println!("{}",rpt)
            }
        }
        for speed in [2,3,5]
        {
            for quality in [85,90,95]
            {
                let sw = sw::Stopwatch::start_new();
                let _ = codecs::encoder::encode_to_avif(&rgb_frames[2].0, rgb_frames[2].1,rgb_frames[2].2, quality as f32, speed,num_cpus::get())?;
                let rpt = format!("encode <photography> quality = {} , speed = {} , use time : +{}sec\n",quality,speed,sw.s());
                log.push_str(rpt.as_str());println!("{}",rpt)
            }
        }
    }
    {//muti photo encode
        let sw = sw::Stopwatch::start_new();
        async fn thread_encode(rgb_frame: (Vec<u8>, u32, u32),quality: f32,speed: u8)->Result<(),Box<dyn Error>>
        {
            let _ = codecs::encoder::encode_to_avif(&rgb_frame.0, rgb_frame.1,rgb_frame.2, quality, speed,num_cpus::get())?;
            Ok(())
        }
        let mut vthread = vec![];  
        for _ in 0..16
        {
            let irgb_frame = rgb_frames[2].clone();
            vthread.push(tokio::spawn(async move
            {
                thread_encode(irgb_frame, 90.0, 5).await.unwrap()
            }))
        }
        for i0 in vthread
        {
            let _ = i0.await?;
        }
        let itime = sw.s();
        let rpt = format!("encode 16 photos , use time : +{}sec\n",sw.s());
                log.push_str(rpt.as_str());println!("{}",rpt)
    }
    std::fs::write("log.txt", log)?;
    Ok(())
}