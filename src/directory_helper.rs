use chrono::{DateTime, FixedOffset, Utc};
use std::{fs, path::Path, time::UNIX_EPOCH};

pub fn get_extension_by_path(path: &Path) -> String {
    if let Some(extension) = path.extension() {
        // OsStrをStringに変換
        if let Some(ext_str) = extension.to_str() {
            return ext_str.to_string();
        } else {
            //println!("{} - 拡張子の変換に失敗", path.display());
            return "error".to_string();
        }
    } else {
        //println!("{} - 拡張子なし", path.display());
        return "none".to_string();
    }
}

pub fn get_created_date_by_path(path: &Path) -> String {
    let jst_offset = FixedOffset::east_opt(9 * 3600).unwrap();
    // メタデータを取得
    if let Ok(metadata) = fs::metadata(path) {
        // 作成日を取得
        if let Ok(created) = metadata.created() {
            // UNIX時間をDateTimeに変換 (UTC)
            let duration = created.duration_since(UNIX_EPOCH).unwrap();
            let datetime_utc = DateTime::<Utc>::from(UNIX_EPOCH + duration);

            // UTCから日本時間（JST）に変換
            let datetime_jst = datetime_utc.with_timezone(&jst_offset);
            let formatted_date = datetime_jst.format("%Y-%m-%d %H:%M:%S").to_string();

            //println!("{} - 作成日 (JST): {}", path.display(), formatted_date);
            return formatted_date;
        } else {
            //println!("{} - 作成日: 取得不可", path.display());
            return "error".to_string();
        }
    } else {
        //println!("{} - メタデータ取得エラー", path.display());
        return "error".to_string();
    }
}

pub fn get_modified_date_by_path(path: &Path) -> String {
    let jst_offset = FixedOffset::east_opt(9 * 3600).unwrap();
    // メタデータを取得
    if let Ok(metadata) = fs::metadata(path) {
        // 作成日を取得
        if let Ok(created) = metadata.modified() {
            // UNIX時間をDateTimeに変換 (UTC)
            let duration = created.duration_since(UNIX_EPOCH).unwrap();
            let datetime_utc = DateTime::<Utc>::from(UNIX_EPOCH + duration);

            // UTCから日本時間（JST）に変換
            let datetime_jst = datetime_utc.with_timezone(&jst_offset);
            let formatted_date = datetime_jst.format("%Y-%m-%d %H:%M:%S").to_string();

            //println!("{} - 作成日 (JST): {}", path.display(), formatted_date);
            return formatted_date;
        } else {
            //println!("{} - 更新日: 取得不可", path.display());
            return "error".to_string();
        }
    } else {
        //println!("{} - メタデータ取得エラー", path.display());
        return "error".to_string();
    }
}
