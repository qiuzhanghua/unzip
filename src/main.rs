use std::fs;
use std::io;

// copied from examples of zip crate
fn main() {
    std::process::exit(real_main());
}

fn real_main() -> i32 {
    // let args: Vec<_> = std::env::args().collect();
    // if args.len() < 2 {
    //     println!("Usage: {} <filename>", args[0]);
    //     return 1;
    // }
    // let fname = std::path::Path::new(&*args[1]);
    let fname = "chinese.zip";
    let file = fs::File::open(&fname).unwrap();

    let mut archive = zip::ZipArchive::new(file).unwrap();

    for i in 0..archive.len() {
        let mut file = archive.by_index(i).unwrap();
        // println!("{}", file.name());
        let outpath = match file.enclosed_name() {
            Some(path) => path.to_owned(),
            None => continue,
        };

        let comment = file.comment();
        if !comment.is_empty() {
            println!("File {} comment: {}", i, comment);
        }

        if (&*file.name()).ends_with('/') {
            println!("File {} extracted to \"{}\"", i, outpath.display());
            fs::create_dir_all(&outpath).unwrap();
        } else {
            println!(
                "File {} extracted to \"{}\" ({} bytes)",
                i,
                outpath.display(),
                file.size()
            );
            if let Some(p) = outpath.parent() {
                if !p.exists() {
                    fs::create_dir_all(&p).unwrap();
                }
            }
            let mut outfile = fs::File::create(&outpath).unwrap();
            io::copy(&mut file, &mut outfile).unwrap();
        }

        // Get and Set permissions
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;

            if let Some(mode) = file.unix_mode() {
                fs::set_permissions(&outpath, fs::Permissions::from_mode(mode)).unwrap();
            }
        }
    }
    return 0;
}


#[cfg(test)]
mod tests {

    #[test]
    fn test_encoding() {
        let origin = "τ¢«σ╜ò"; // "目录";
        let s = origin.as_bytes();
        println!("{:?}", origin.chars());
        println!("{:?}", s.len());
        for c in s {
            println!("{:08b}", c);
        }
 //       let e = origin.encode_utf16();
        println!();
        for c in "目录".chars()  {
            println!("{:016b}", c as u16);
        }
        for c in "目录".as_bytes()  {
            println!("{:08b}", c);
        }

        let ch ='目' as i32;
        let ch_unicode = format!("{:X}", ch);
        println!("ch:{:?}", ch_unicode);
        assert_eq!(1 + 2, 3);
    }
}