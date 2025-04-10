use reqwest::{Error,StatusCode};
use serde::{Deserialize};
use std::fs;
use std::collections::HashMap;
async fn npmrepo(url: &str) -> Result<(), Error>{
    let fullurl = format!("https://registry.npmjs.org/{}", url);
    let response = reqwest::get(fullurl).await?;
    if response.status() == StatusCode::NOT_FOUND{
        println!("{}",url);
    }
    Ok(())
}



#[derive(Deserialize, Debug)]
struct PackageJson {
    dependencies: Option<HashMap<String, String>>,     
    devDependencies: Option<HashMap<String, String>>,  
    peerDependencies: Option<HashMap<String, String>>, 
    optionalDependencies: Option<HashMap<String, String>>, 
}
pub async fn NpmCheck(filePath: &str) -> Result<(), Box<dyn std::error::Error>> {
    let content = fs::read_to_string(filePath)?;

    let package: PackageJson = serde_json::from_str(&content)?;

    if let Some(deps) = package.dependencies {
        for (name, version) in deps {
            npmrepo(&name).await;
        }
    }
    
    if let Some(deps) = package.devDependencies {
        for (name, version) in deps {
            npmrepo(&name).await;
        }
    }
    
    if let Some(deps) = package.optionalDependencies {
        for (name, version) in deps {
            npmrepo(&name).await;
        }
    }
    
    if let Some(deps) = package.peerDependencies {
        for (name, version) in deps {
            npmrepo(&name).await;
        }
    }

    Ok(())
}
