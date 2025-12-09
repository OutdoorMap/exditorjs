/// Embed service detection and parsing utilities
use regex::Regex;

/// Represents an embed service configuration
#[derive(Debug, Clone)]
pub struct EmbedService {
    pub name: &'static str,
    pub regex: Regex,
    pub embed_url_template: &'static str,
    pub width: u32,
    pub height: u32,
}

lazy_static::lazy_static! {
    static ref EMBED_SERVICES: Vec<EmbedService> = vec![
        EmbedService {
            name: "youtube",
            regex: Regex::new(r"(?:https?://)?(?:www\.)?(?:youtube\.com/watch\?v=|youtu\.be/)([a-zA-Z0-9_-]{11})").unwrap(),
            embed_url_template: "https://www.youtube.com/embed/{}",
            width: 580,
            height: 320,
        },
        EmbedService {
            name: "vimeo",
            regex: Regex::new(r"(?:https?://)?(?:www\.)?vimeo\.com/(\d+)").unwrap(),
            embed_url_template: "https://player.vimeo.com/video/{}",
            width: 580,
            height: 320,
        },
        EmbedService {
            name: "coub",
            regex: Regex::new(r"(?:https?://)?(?:www\.)?coub\.com/view/([a-zA-Z0-9]+)").unwrap(),
            embed_url_template: "https://coub.com/embed/{}",
            width: 580,
            height: 320,
        },
        EmbedService {
            name: "instagram",
            regex: Regex::new(r"(?:https?://)?(?:www\.)?instagram\.com/(?:p|reel)/([a-zA-Z0-9_-]+)").unwrap(),
            embed_url_template: "https://www.instagram.com/p/{}/embed/",
            width: 540,
            height: 663,
        },
        EmbedService {
            name: "twitter",
            regex: Regex::new(r"(?:https?://)?(?:www\.)?(?:twitter\.com|x\.com)/\w+/status/(\d+)").unwrap(),
            embed_url_template: "https://twitter.com/i/web/status/{}",
            width: 550,
            height: 300,
        },
        EmbedService {
            name: "twitch-video",
            regex: Regex::new(r"(?:https?://)?(?:www\.)?twitch\.tv/videos/(\d+)").unwrap(),
            embed_url_template: "https://player.twitch.tv/?video={}",
            width: 500,
            height: 281,
        },
        EmbedService {
            name: "twitch-channel",
            regex: Regex::new(r"(?:https?://)?(?:www\.)?twitch\.tv/([a-zA-Z0-9_]+)(?:/)?$").unwrap(),
            embed_url_template: "https://player.twitch.tv/?channel={}",
            width: 500,
            height: 281,
        },
        EmbedService {
            name: "codepen",
            regex: Regex::new(r"(?:https?://)?(?:www\.)?codepen\.io/([^/]+)/pen/([a-zA-Z0-9]+)").unwrap(),
            embed_url_template: "https://codepen.io/{}/embed/{}",
            width: 600,
            height: 300,
        },
        EmbedService {
            name: "github",
            regex: Regex::new(r"(?:https?://)?(?:www\.)?gist\.github\.com/([^/]+)/([a-zA-Z0-9]+)").unwrap(),
            embed_url_template: "https://gist.github.com/{}/{}",
            width: 600,
            height: 300,
        },
        EmbedService {
            name: "figma",
            regex: Regex::new(r"(?:https?://)?(?:www\.)?figma\.com/(?:file|proto)/([a-zA-Z0-9]+)").unwrap(),
            embed_url_template: "https://www.figma.com/embed?embed_host=share&url=https://www.figma.com/file/{}",
            width: 800,
            height: 450,
        },
        EmbedService {
            name: "miro",
            regex: Regex::new(r"(?:https?://)?(?:www\.)?miro\.com/app/board/([a-zA-Z0-9_-]+)").unwrap(),
            embed_url_template: "https://miro.com/app/board/{}/",
            width: 800,
            height: 600,
        },
        EmbedService {
            name: "imgur",
            regex: Regex::new(r"(?:https?://)?(?:www\.)?imgur\.com/([a-zA-Z0-9]+)").unwrap(),
            embed_url_template: "https://imgur.com/{}/embed",
            width: 540,
            height: 500,
        },
        EmbedService {
            name: "pinterest",
            regex: Regex::new(r"(?:https?://)?(?:www\.)?pinterest\.com/pin/(\d+)").unwrap(),
            embed_url_template: "https://www.pinterest.com/pin/{}/",
            width: 520,
            height: 600,
        },
    ];
}

/// Detect if a URL matches any known embed service
pub fn detect_embed_service(url: &str) -> Option<(String, String, u32, u32)> {
    let url = url.trim();

    for service in EMBED_SERVICES.iter() {
        if let Some(caps) = service.regex.captures(url) {
            let id = if caps.len() > 2 {
                // For services with multiple capture groups
                let mut parts = vec![];
                for i in 1..caps.len() {
                    if let Some(m) = caps.get(i) {
                        parts.push(m.as_str().to_string());
                    }
                }
                parts.join("/")
            } else {
                // For services with single capture group
                caps.get(1)
                    .map(|m| m.as_str().to_string())
                    .unwrap_or_default()
            };

            if !id.is_empty() {
                let embed_url = service.embed_url_template.replace("{}", &id);
                return Some((
                    service.name.to_string(),
                    embed_url,
                    service.width,
                    service.height,
                ));
            }
        }
    }

    None
}

/// Parse an iframe tag and extract embed information
pub fn parse_iframe(attrs: &str) -> Option<(String, u32, u32)> {
    let src_re = Regex::new(r#"src=["']?([^"'\s>]+)["']?"#).ok()?;
    let width_re = Regex::new(r#"width=["']?(\d+)["']?"#).ok()?;
    let height_re = Regex::new(r#"height=["']?(\d+)["']?"#).ok()?;

    let src = src_re
        .captures(attrs)
        .and_then(|cap| cap.get(1))
        .map(|m| m.as_str())?;

    let width = width_re
        .captures(attrs)
        .and_then(|cap| cap.get(1))
        .and_then(|m| m.as_str().parse().ok())
        .unwrap_or(580);

    let height = height_re
        .captures(attrs)
        .and_then(|cap| cap.get(1))
        .and_then(|m| m.as_str().parse().ok())
        .unwrap_or(320);

    Some((src.to_string(), width, height))
}

/// Detect the embed service from an iframe src URL
pub fn detect_service_from_src(src: &str) -> Option<String> {
    if src.contains("youtube.com") || src.contains("youtu.be") {
        Some("youtube".to_string())
    } else if src.contains("vimeo.com") {
        Some("vimeo".to_string())
    } else if src.contains("coub.com") {
        Some("coub".to_string())
    } else if src.contains("instagram.com") {
        Some("instagram".to_string())
    } else if src.contains("twitter.com") || src.contains("x.com") {
        Some("twitter".to_string())
    } else if src.contains("twitch.tv") {
        if src.contains("video=") {
            Some("twitch-video".to_string())
        } else {
            Some("twitch-channel".to_string())
        }
    } else if src.contains("codepen.io") {
        Some("codepen".to_string())
    } else if src.contains("gist.github.com") {
        Some("github".to_string())
    } else if src.contains("figma.com") {
        Some("figma".to_string())
    } else if src.contains("miro.com") {
        Some("miro".to_string())
    } else if src.contains("imgur.com") {
        Some("imgur".to_string())
    } else if src.contains("pinterest.com") {
        Some("pinterest".to_string())
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_detect_youtube_url() {
        let url = "https://www.youtube.com/watch?v=dQw4w9WgXcQ";
        let result = detect_embed_service(url);
        assert!(result.is_some());
        let (service, embed_url, _, _) = result.unwrap();
        assert_eq!(service, "youtube");
        assert!(embed_url.contains("dQw4w9WgXcQ"));
    }

    #[test]
    fn test_detect_youtube_short_url() {
        let url = "https://youtu.be/dQw4w9WgXcQ";
        let result = detect_embed_service(url);
        assert!(result.is_some());
        let (service, _, _, _) = result.unwrap();
        assert_eq!(service, "youtube");
    }

    #[test]
    fn test_detect_vimeo_url() {
        let url = "https://vimeo.com/123456789";
        let result = detect_embed_service(url);
        assert!(result.is_some());
        let (service, _, _, _) = result.unwrap();
        assert_eq!(service, "vimeo");
    }

    #[test]
    fn test_detect_coub_url() {
        let url = "https://coub.com/view/1czcdf";
        let result = detect_embed_service(url);
        assert!(result.is_some());
        let (service, _, _, _) = result.unwrap();
        assert_eq!(service, "coub");
    }

    #[test]
    fn test_detect_instagram_url() {
        let url = "https://www.instagram.com/p/ABC123XYZ/";
        let result = detect_embed_service(url);
        assert!(result.is_some());
        let (service, _, _, _) = result.unwrap();
        assert_eq!(service, "instagram");
    }

    #[test]
    fn test_detect_twitter_url() {
        let url = "https://twitter.com/user/status/1234567890";
        let result = detect_embed_service(url);
        assert!(result.is_some());
        let (service, _, _, _) = result.unwrap();
        assert_eq!(service, "twitter");
    }

    #[test]
    fn test_detect_invalid_url() {
        let url = "https://example.com/some/page";
        let result = detect_embed_service(url);
        assert!(result.is_none());
    }

    #[test]
    fn test_parse_iframe() {
        let attrs = r#"src="https://www.youtube.com/embed/dQw4w9WgXcQ" width="560" height="315""#;
        let result = parse_iframe(attrs);
        assert!(result.is_some());
        let (src, width, height) = result.unwrap();
        assert!(src.contains("dQw4w9WgXcQ"));
        assert_eq!(width, 560);
        assert_eq!(height, 315);
    }

    #[test]
    fn test_detect_service_from_src() {
        let src = "https://www.youtube.com/embed/dQw4w9WgXcQ";
        let result = detect_service_from_src(src);
        assert_eq!(result, Some("youtube".to_string()));
    }
}
