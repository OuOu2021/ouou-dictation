#[cfg(test)]
mod test_tts {
    use anyhow::Result;
    use tts::{Gender, LanguageTag};

    #[test]
    fn test_read() -> Result<()> {
        let mut speaker = tts::Tts::default().unwrap();
        speaker
            .on_utterance_begin(Some(Box::new(|_| {
                println!("begin speaking");
            })))
            .expect("设置回调函数失败");
        speaker
            .on_utterance_end(Some(Box::new(|_| {
                println!("finished");
            })))
            .expect("设置回调函数失败");
        speaker.speak("hello world", false).unwrap();

        let voices = speaker.voices().unwrap();

        speaker
            .set_voice(
                &voices
                    .iter()
                    .find(|&a| {
                        a.language() == LanguageTag::parse("zh-CN").expect("解析失败")
                            && a.gender() == Some(Gender::Female)
                    })
                    .unwrap(),
            )
            .unwrap();
        speaker.speak("你好", false).unwrap();

        println!("回车以结束运行");
        let mut _str = String::new();
        std::io::stdin().read_line(&mut _str)?;
        Ok(())
    }
}
