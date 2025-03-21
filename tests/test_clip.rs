use clipboard_win::formats::{
    Bitmap, FileList, RawData, Unicode, CF_BITMAP, CF_HDROP, CF_TEXT, CF_UNICODETEXT,
};
use clipboard_win::{is_format_avail, Clipboard, Getter, Setter};

fn should_set_file_list() {
    let _clip = Clipboard::new_attempts(10).expect("Open clipboard");
    // Note that you will not be able to paste the paths below in Windows Explorer because Explorer
    // does not play nice with canonicalize: https://github.com/rust-lang/rust/issues/42869.
    // Pasting in Explorer works fine with regular, non-UNC paths.
    let paths = [
        std::fs::canonicalize("tests/test-image.bmp")
            .expect("to get abs path")
            .display()
            .to_string(),
        std::fs::canonicalize("tests/formats.rs")
            .expect("to get abs path")
            .display()
            .to_string(),
    ];
    FileList
        .write_clipboard(&paths, true)
        .expect("set file to copy");

    let mut set_files = Vec::<String>::with_capacity(2);
    FileList.read_clipboard(&mut set_files).expect("read");
    assert_eq!(set_files, paths);
}

fn should_work_with_bitmap() {
    let _clip = Clipboard::new_attempts(10).expect("Open clipboard");

    let test_image_bytes = std::fs::read("tests/test-image.bmp").expect("Read test image");
    Bitmap
        .write_clipboard(&test_image_bytes, true)
        .expect("To set image");

    let mut out = Vec::new();

    assert_eq!(
        Bitmap.read_clipboard(&mut out).expect("To get image"),
        out.len()
    );

    assert_eq!(test_image_bytes.len(), out.len());
    assert!(test_image_bytes == out);
}

fn should_work_with_string() {
    let text = "For my waifu\n!";

    let _clip = Clipboard::new_attempts(10).expect("Open clipboard");

    Unicode.write_clipboard(&text, true).expect("Write text");

    let mut output = String::new();

    assert_eq!(
        Unicode.read_clipboard(&mut output).expect("Read text"),
        text.len()
    );
    assert_eq!(text, output);

    assert_eq!(
        Unicode.read_clipboard(&mut output).expect("Read text"),
        text.len()
    );
    assert_eq!(format!("{0}{0}", text), output);
}

fn should_work_with_wide_string() {
    let text = "メヒーシャ!";

    let _clip = Clipboard::new_attempts(10).expect("Open clipboard");

    Unicode.write_clipboard(&text, true).expect("Write text");

    let mut output = String::new();

    assert_eq!(
        Unicode.read_clipboard(&mut output).expect("Read text"),
        text.len()
    );
    assert_eq!(text, output);

    assert_eq!(
        Unicode.read_clipboard(&mut output).expect("Read text"),
        text.len()
    );
    assert_eq!(format!("{0}{0}", text), output);
}

fn should_work_with_bytes() {
    let text = "Again waifu!?\0";

    let ascii = RawData(CF_TEXT);
    let _clip = Clipboard::new_attempts(10).expect("Open clipboard");

    ascii.write_clipboard(&text, true).expect("Write ascii");

    let mut output = String::with_capacity(text.len() * 2);

    {
        let output = unsafe { output.as_mut_vec() };
        assert_eq!(
            ascii.read_clipboard(output).expect("read ascii"),
            text.len()
        );
    }

    assert_eq!(text, output);

    {
        let output = unsafe { output.as_mut_vec() };
        assert_eq!(
            ascii.read_clipboard(output).expect("read ascii"),
            text.len()
        );
    }

    assert_eq!(format!("{0}{0}", text), output);
}

fn should_work_with_set_empty_string() {
    let text = "";

    let _clip = Clipboard::new_attempts(10).expect("Open clipboard");

    Unicode.write_clipboard(&text, true).expect("Write text");

    let mut output = String::new();

    assert_eq!(
        Unicode.read_clipboard(&mut output).expect("Read text"),
        text.len()
    );
    assert_eq!(text, output);
}

extern "system" {
    fn GetConsoleWindow() -> winapi::shared::windef::HWND;
}

fn should_set_owner() {
    {
        assert!(clipboard_win::get_owner().is_none());
        let _clip = Clipboard::new_attempts(10).expect("Open clipboard");
        assert!(clipboard_win::get_owner().is_none());
    }

    let console = unsafe { GetConsoleWindow() };
    if !console.is_null() {
        let _clip = Clipboard::new_attempts_for(console, 10).expect("Open clipboard");
        let _ = clipboard_win::empty(); //empty is necessary to finalize association
        assert_eq!(
            clipboard_win::get_owner().expect("to have owner").as_ptr() as usize,
            console as usize
        );
    }
}

macro_rules! run {
    ($name:ident) => {
        println!("Clipboard test: {}...", stringify!($name));
        $name();
    };
}

#[test]
fn clipboard_should_work() {
    run!(should_work_with_bitmap);
    assert!(is_format_avail(CF_BITMAP));
    run!(should_work_with_string);
    assert!(is_format_avail(CF_UNICODETEXT));
    run!(should_set_file_list);
    assert!(is_format_avail(CF_HDROP));
    run!(should_work_with_wide_string);
    run!(should_work_with_bytes);
    run!(should_work_with_set_empty_string);
    run!(should_set_owner);
}
