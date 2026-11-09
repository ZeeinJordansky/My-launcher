use std::path::PathBuf;
use std::process::Command;

pub fn find_java(preferred: Option<&str>) -> Option<PathBuf> {
    if let Some(p) = preferred {
        let path = PathBuf::from(p);
        if path.exists() {
            return Some(path);
        }
    }

    for name in ["java", "javaw"] {
        if let Ok(path) = which::which(name) {
            return Some(path);
        }
    }

    let candidates = [
        r"C:\Program Files\Java\jdk-21\bin\java.exe",
        r"C:\Program Files\Java\jdk-17\bin\java.exe",
        r"C:\Program Files\Eclipse Adoptium\jdk-21\bin\java.exe",
        r"C:\Program Files\Eclipse Adoptium\jdk-17\bin\java.exe",
        "/usr/lib/jvm/default-java/bin/java",
        "/usr/lib/jvm/java-21-openjdk/bin/java",
        "/usr/lib/jvm/java-17-openjdk/bin/java",
        "/usr/bin/java",
    ];

    for c in candidates {
        let p = PathBuf::from(c);
        if p.exists() {
            return Some(p);
        }
    }

    None
}

pub fn java_version(java: &PathBuf) -> Option<String> {
    let output = Command::new(java).arg("-version").output().ok()?;
    let stderr = String::from_utf8_lossy(&output.stderr);
    stderr.lines().next().map(|s| s.to_string())
}
