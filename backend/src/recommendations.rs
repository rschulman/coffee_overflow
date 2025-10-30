use axum::{Json, extract::State, http::StatusCode};
use serde::{Deserialize, Serialize};
use tower_cookies::Cookies;
use sea_orm::{EntityTrait, ColumnTrait, QueryFilter};
use std::env;

#[derive(Deserialize)]
pub struct RecommendationsRequest {
    pub interests: String,
}

#[derive(Serialize, Deserialize)]
pub struct CourseRecommendation {
    pub title: String,
    pub provider: String,
    pub hours: f32,
    pub topic: String,
    pub format: String,
    pub price: String,
    pub url: String,
    pub ai_reason: String,
}

#[derive(Serialize)]
pub struct RecommendationsResponse {
    pub recommendations: Vec<CourseRecommendation>,
}

// Sanitize user input to prevent prompt injection and XSS
fn sanitize_input(input: &str) -> String {
    // 1. Limit length
    let truncated = input.chars().take(200).collect::<String>();

    // 2. Remove control characters and HTML tags
    let cleaned = truncated
        .chars()
        .filter(|c| !c.is_control() && *c != '<' && *c != '>')
        .collect::<String>();

    // 3. Remove prompt injection patterns
    let safe = cleaned
        .replace("ignore previous", "")
        .replace("ignore all", "")
        .replace("system:", "")
        .replace("assistant:", "")
        .replace("user:", "")
        .replace("```", "");

    // 4. Trim whitespace
    safe.trim().to_string()
}

pub async fn get_recommendations(
    state: State<crate::AppState>,
    cookies: Cookies,
    Json(request): Json<RecommendationsRequest>,
) -> Result<Json<RecommendationsResponse>, (StatusCode, &'static str)> {
    // Verify session
    let session_token = cookies
        .get("session")
        .ok_or((StatusCode::UNAUTHORIZED, "Not logged in"))?
        .value()
        .to_string();

    let user = entity::user::Entity::find()
        .inner_join(entity::session::Entity)
        .filter(entity::session::Column::Token.eq(session_token))
        .one(&state.conn)
        .await
        .map_err(|_| (StatusCode::INTERNAL_SERVER_ERROR, "Database error"))?
        .ok_or((StatusCode::FORBIDDEN, "Not logged in"))?;

    // Get user's states
    let user_states = entity::user_state::Entity::find()
        .filter(entity::user_state::Column::UserId.eq(user.id))
        .all(&state.conn)
        .await
        .map_err(|_| (StatusCode::INTERNAL_SERVER_ERROR, "Database error"))?;

    let total_hours_needed: i32 = user_states.iter()
        .map(|s| s.hours_complete)
        .sum();

    // Sanitize user interests
    let safe_interests = sanitize_input(&request.interests);

    // Get Gemini API key
    let api_key = env::var("GEMINI_API_KEY").unwrap_or_default();

    let recommendations = if api_key.is_empty() || safe_interests.is_empty() {
        get_fallback_recommendations()
    } else {
        let start = std::time::Instant::now();
        match call_gemini_api(&api_key, &user.fullname, total_hours_needed, &safe_interests).await {
            Ok(recs) => {
                println!("Gemini API call took {:?}", start.elapsed());
                recs
            },
            Err(e) => {
                eprintln!("Gemini API error: {}", e);
                get_fallback_recommendations()
            }
        }
    };

    Ok(Json(RecommendationsResponse { recommendations }))
}

fn get_fallback_recommendations() -> Vec<CourseRecommendation> {
    vec![
        CourseRecommendation {
            title: "Professional Responsibility and Ethics".to_string(),
            provider: "Massachusetts CLE".to_string(),
            hours: 2.0,
            topic: "Ethics".to_string(),
            format: "Online".to_string(),
            price: "$50".to_string(),
            url: "https://www.massbar.org/continuing-legal-education".to_string(),
            ai_reason: "Essential ethics training for maintaining professional standards".to_string(),
        },
        CourseRecommendation {
            title: "Introduction to Legal Technology Tools".to_string(),
            provider: "ABA Techshow".to_string(),
            hours: 1.5,
            topic: "Legal Tech".to_string(),
            format: "Online".to_string(),
            price: "Free".to_string(),
            url: "https://www.techshow.com/education".to_string(),
            ai_reason: "Learn practical technology tools for modern legal practice".to_string(),
        },
        CourseRecommendation {
            title: "Contract Drafting Essentials".to_string(),
            provider: "Practising Law Institute".to_string(),
            hours: 3.0,
            topic: "Contract Law".to_string(),
            format: "Self-Paced".to_string(),
            price: "$99".to_string(),
            url: "https://www.pli.edu/programs/contract-drafting".to_string(),
            ai_reason: "Build strong foundation in contract drafting skills".to_string(),
        },
        CourseRecommendation {
            title: "Business Law Foundations".to_string(),
            provider: "Coursera".to_string(),
            hours: 4.0,
            topic: "Business Law".to_string(),
            format: "Self-Paced".to_string(),
            price: "Free".to_string(),
            url: "https://www.coursera.org/courses?query=business%20law".to_string(),
            ai_reason: "Comprehensive overview of key business law concepts".to_string(),
        },
    ]
}

fn select_courses_by_keywords(interests: &str) -> Vec<(String, String, f32, String, String, String, String)> {
    let catalog = vec![
        ("Professional Responsibility and Ethics", "Massachusetts CLE", 2.0, "Ethics", "Online", "$50", "https://www.massbar.org/continuing-legal-education"),
        ("Contract Drafting Essentials", "Practising Law Institute", 3.0, "Contract Law", "Self-Paced", "$99", "https://www.pli.edu/programs/contract-drafting"),
        ("Introduction to Legal Technology Tools", "ABA Techshow", 1.5, "Legal Tech", "Online", "Free", "https://www.techshow.com/education"),
        ("Florida Bar Ethics Update 2024", "Florida Bar CLE", 2.5, "Ethics", "Live Webinar", "Free", "https://www.floridabar.org/cle/"),
        ("AI for Legal Professionals", "Stanford CodeX", 2.0, "Legal Tech", "Online", "Free", "https://law.stanford.edu/codex-the-stanford-center-for-legal-informatics/"),
        ("Advanced Contract Negotiation", "Harvard Law School", 3.5, "Contract Law", "Self-Paced", "$150", "https://online-learning.harvard.edu/catalog"),
        ("Data Privacy and GDPR Compliance", "IAPP", 2.0, "Privacy Law", "Online", "$200", "https://iapp.org/store/courses/"),
        ("Legal Writing for Clarity", "Legal Writing Institute", 1.5, "Legal Writing", "Self-Paced", "Free", "https://www.lwionline.org/"),
        ("Business Law Foundations", "Coursera", 4.0, "Business Law", "Self-Paced", "Free", "https://www.coursera.org/courses?query=business%20law"),
        ("Immigration Law Essentials", "AILA", 2.5, "Immigration", "Live Webinar", "$125", "https://www.aila.org/cle"),
        ("Federal Taxation Fundamentals", "NYU School of Law", 3.0, "Tax Law", "Self-Paced", "$175", "https://www.law.nyu.edu/academics/cle"),
        ("Real Estate Transactions", "California Lawyers Association", 2.5, "Real Estate Law", "Online", "$95", "https://calawyers.org/cle/"),
        ("Divorce and Child Custody Essentials", "National Business Institute", 3.0, "Family Law", "Live Webinar", "$149", "https://www.nbi-sems.com/"),
        ("Family Law Practice Fundamentals", "State Bar of Texas", 2.5, "Family Law", "Online", "$125", "https://www.texasbar.com/"),
        ("Bankruptcy Law Basics", "American Bankruptcy Institute", 2.0, "Bankruptcy Law", "Self-Paced", "$150", "https://www.abi.org/"),
        ("Personal Injury Litigation", "National Institute for Trial Advocacy", 3.5, "Personal Injury", "Online", "$200", "https://www.nita.org/"),
        ("Criminal Defense Strategies", "NACDL", 3.0, "Criminal Law", "Live Webinar", "$150", "https://www.nacdl.org/cle/"),
        ("Employment Discrimination Law", "Georgetown Law", 3.5, "Employment Law", "Self-Paced", "$200", "https://www.law.georgetown.edu/continuing-legal-education/"),
        ("Civil Litigation Fundamentals", "American Bar Association", 2.5, "Civil Litigation", "Online", "$125", "https://www.americanbar.org/cle/"),
        ("Intellectual Property Overview", "American Bar Association", 2.5, "IP Law", "Online", "$125", "https://www.americanbar.org/cle/"),
        ("Estate Planning and Wills", "UC Berkeley Extension", 3.0, "Estate Planning", "Online", "$195", "https://extension.berkeley.edu/"),
        ("Healthcare Compliance Essentials", "American Health Law Association", 2.0, "Healthcare Law", "Online", "$175", "https://www.americanbar.org/groups/health_law/"),
        ("Environmental Law Basics", "Environmental Law Institute", 2.5, "Environmental Law", "Self-Paced", "$100", "https://www.eli.org/"),
        ("Cybersecurity for Law Firms", "ILTA", 1.5, "Legal Tech", "Online", "Free", "https://www.iltanet.org/"),
    ];

    let interests_lower = interests.to_lowercase();
    let mut scored_courses: Vec<_> = catalog.iter().map(|course| {
        let title = course.0.to_lowercase();
        let topic = course.3.to_lowercase();
        let mut score = 0;

        // Score based on keyword matches
        for word in interests_lower.split_whitespace() {
            if word.len() < 3 { continue; }
            if title.contains(word) { score += 3; }
            if topic.contains(word) { score += 2; }
        }

        (score, course.clone())
    }).collect();

    scored_courses.sort_by(|a, b| b.0.cmp(&a.0));
    scored_courses.iter().take(4).map(|(_, c)| {
        (c.0.to_string(), c.1.to_string(), c.2, c.3.to_string(), c.4.to_string(), c.5.to_string(), c.6.to_string())
    }).collect()
}

async fn call_gemini_api(
    api_key: &str,
    _user_name: &str,
    _hours_needed: i32,
    interests: &str,
) -> Result<Vec<CourseRecommendation>, Box<dyn std::error::Error>> {
    let client = reqwest::Client::new();

    // Pre-select courses by keyword matching
    let selected = select_courses_by_keywords(interests);

    let courses_text = selected.iter().enumerate()
        .map(|(i, c)| format!("{}. {}", i+1, c.0))
        .collect::<Vec<_>>()
        .join("\n");

    let prompt = format!(
        r#"User interests: "{}"
Courses:
{}

Write a brief ai_reason (10-15 words) for each course explaining why it matches their interests.
Return JSON: ["reason1","reason2","reason3","reason4"]"#,
        interests, courses_text
    );

    let request_body = serde_json::json!({
        "contents": [{
            "parts": [{
                "text": prompt
            }]
        }],
        "generationConfig": {
            "temperature": 0.7,
            "maxOutputTokens": 500
        }
    });

    let response = client
        .post(format!(
            "https://generativelanguage.googleapis.com/v1beta/models/gemini-2.5-flash:generateContent?key={}",
            api_key
        ))
        .json(&request_body)
        .send()
        .await?;

    let status = response.status();
    if !status.is_success() {
        let error_body = response.text().await.unwrap_or_else(|_| "Could not read error body".to_string());
        return Err(format!("API request failed with status {}: {}", status, error_body).into());
    }

    let response_json: serde_json::Value = response.json().await?;
    let text = response_json["candidates"][0]["content"]["parts"][0]["text"]
        .as_str()
        .ok_or("Failed to parse response")?;

    // Strip markdown code blocks if present
    let json_text = text
        .replace("```json", "")
        .replace("```", "")
        .trim()
        .to_string();

    let ai_reasons: Vec<String> = serde_json::from_str(&json_text)?;

    // Combine pre-selected courses with AI reasons
    let recommendations = selected.iter().zip(ai_reasons.iter())
        .map(|(course, reason)| CourseRecommendation {
            title: course.0.clone(),
            provider: course.1.clone(),
            hours: course.2,
            topic: course.3.clone(),
            format: course.4.clone(),
            price: course.5.clone(),
            url: course.6.clone(),
            ai_reason: reason.clone(),
        })
        .collect();

    Ok(recommendations)
}
