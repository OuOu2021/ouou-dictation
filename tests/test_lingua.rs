#[cfg(test)]
mod test_lingua {
    use std::ops::Add;

    use lingua::Language::{Chinese, English, Japanese};
    use lingua::{Language, LanguageDetectorBuilder};
    const LANGUAGES: [Language; 3] = [English, Japanese, Chinese];

    #[test]
    fn test_chinese() {
        let detector = LanguageDetectorBuilder::from_languages(&LANGUAGES).build();
        let chinese_text = ["你好", "我是XXX", "学生"];
        chinese_text.into_iter().for_each(|x| {
            let chinese_lang = detector.detect_language_of(x);
            assert_eq!(chinese_lang, Some(Chinese));
        });
    }

    #[test]
    fn test_japanese() {
        let detector = LanguageDetectorBuilder::from_languages(&LANGUAGES).build();
        let japanese_text = ["こんにちは", "学生です", "ソフトウェア"];
        japanese_text.into_iter().for_each(|x| {
            let japanese_lang = detector.detect_language_of(x);
            assert_eq!(japanese_lang, Some(Japanese));
        });
    }

    #[test]
    fn known_misidentification_test() {
        let detector = LanguageDetectorBuilder::from_languages(&LANGUAGES).build();

        // it seems that both Chinese characters and
        // kanji(Japanese characters based on Chinese symbols,
        // some of them are not used in Chinese at all) will be
        // identified as Chinese

        let japanese_text_misidentified_as_chinese = [
            "経済",
            "和製漢字",
            "雫",
            "労働",
            "峠",
            "勉強中",
            "自動販売機",
        ];
        japanese_text_misidentified_as_chinese
            .into_iter()
            .for_each(|x| {
                let japanese_lang = detector.detect_language_of(x);
                let dis = detector.compute_language_confidence_values(x);

                println!(
                    "{x}:{}",
                    dis.into_iter()
                        .fold(String::new(), |last, now| { last.add(&format!("{now:?}")) })
                );

                assert_ne!(japanese_lang, Some(Japanese));
                assert_eq!(japanese_lang, Some(Chinese));
            });
    }

    #[test]
    fn test_english() {
        let detector = LanguageDetectorBuilder::from_languages(&LANGUAGES).build();
        let english_text = ["hello", "I am a student.", "Is Lingua good enough?"];
        english_text.into_iter().for_each(|x| {
            let english_lang = detector.detect_language_of(x);
            assert_eq!(english_lang, Some(English));
        });
    }
}
