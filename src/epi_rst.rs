use reqwest::Client;
use serde_json;

pub struct EpiRst {
    autologin_token: String,
    client: reqwest::Client,
}

type EpiRstJsonReply = Result<serde_json::Value, Box<dyn std::error::Error>>;

impl EpiRst {
    fn get_payload(&self, endpoint: &str) -> String {
        let res = String::from("https://intra.epitech.eu/")
            + &self.autologin_token
            + "/"
            + endpoint
            + if endpoint.contains("?") { "&" } else { "?" }
            + "format=json";
        println!("Requesting: {}", res);
        res
    }

    async fn post(&self, endpoint: &str) -> EpiRstJsonReply {
        let res = self
            .client
            .post(self.get_payload(endpoint))
            .send()
            .await?
            .text()
            .await?;
        Ok(serde_json::from_str(&res)?)
    }

    async fn get(&self, endpoint: &str) -> EpiRstJsonReply {
        let res = self
            .client
            .get(self.get_payload(endpoint))
            .send()
            .await?
            .text()
            .await?;
        Ok(serde_json::from_str(&res)?)
    }

    pub async fn get_dashboard(&self) -> EpiRstJsonReply {
        Ok(self.get("").await?)
    }

    pub async fn get_user(&self, user: &str) -> EpiRstJsonReply {
        Ok(self.get(&format!("user/{user}")).await?)
    }

    pub async fn get_user_netsoul(&self, user: &str) -> EpiRstJsonReply {
        Ok(self.get(&format!("user/{user}/netsoul")).await?)
    }

    pub async fn get_partners(&self, user: &str) -> EpiRstJsonReply {
        Ok(self.get(&format!("user/{user}/binome")).await?)
    }

    pub async fn get_user_absences(&self, user: &str) -> EpiRstJsonReply {
        Ok(self.get(&format!("user/{user}/absences")).await?)
    }

    pub async fn get_planning(&self) -> EpiRstJsonReply {
        Ok(self.get("planning/load").await?)
    }

    pub async fn get_module_board(&self, module: &str) -> EpiRstJsonReply {
        Ok(self.get(&format!("module/{module}/board")).await?)
    }

    pub async fn course_filter(&self) -> EpiRstJsonReply {
        Ok(self.get("course/filter").await?)
    }

    pub async fn get_module(&self, year: &str, module: &str, instance: &str) -> EpiRstJsonReply {
        Ok(self
            .get(&format!("module/{year}/{module}/{instance}"))
            .await?)
    }

    pub async fn get_registered_module(
        self,
        year: &str,
        module: &str,
        instance: &str,
    ) -> EpiRstJsonReply {
        Ok(self
            .get(&format!(
                "module/{year}/{module}/{instance}/registered"
            ))
            .await?)
    }

    pub async fn get_project(
        &self,
        year: &str,
        module: &str,
        instance: &str,
        activity: &str,
    ) -> EpiRstJsonReply {
        Ok(self
            .get(&format!(
                "module/{year}/{module}/{instance}/{activity}/project"
            ))
            .await?)
    }

    pub async fn get_project_registered(
        &self,
        year: &str,
        module: &str,
        instance: &str,
        activity: &str,
    ) -> EpiRstJsonReply {
        Ok(self
            .get(&format!(
                "module/{}/{}/{}/{}/project/registered",
                year, module, instance, activity
            ))
            .await?)
    }

    pub async fn get_project_unregistered(
        &self,
        year: &str,
        module: &str,
        instance: &str,
        activity: &str,
    ) -> Result<Vec<String>, Box<dyn std::error::Error>> {
        Ok(self
            .client
            .get(self.get_payload(&format!(
                "module/{}/{}/{}/{}/project/unregistered",
                year, module, instance, activity
            )))
            .send()
            .await?
            .text()
            .await?
            .split("\n")
            .map(|x| x.to_string())
            .collect())
    }

    pub async fn download_file(&self, url: &str) -> Result<String, Box<dyn std::error::Error>> {
        Ok(self
            .client
            .get(self.get_payload(url))
            .send()
            .await?
            .text()
            .await?)
    }

    pub async fn get_activity(
        &self,
        year: &str,
        module: &str,
        instance: &str,
        activity: &str,
    ) -> EpiRstJsonReply {
        Ok(self
            .get(&format!(
                "module/{}/{}/{}/{}",
                year, module, instance, activity
            ))
            .await?)
    }

    pub async fn get_project_files(
        &self,
        year: &str,
        module: &str,
        instance: &str,
        activity: &str,
        event: &str,
    ) -> EpiRstJsonReply {
        Ok(self
            .get(&format!(
                "module/{}/{}/{}/{}/{}/project/file/",
                year, module, instance, activity, event
            ))
            .await?)
    }

    pub async fn get_event_registered(
        &self,
        year: &str,
        module: &str,
        instance: &str,
        activity: &str,
        event: &str,
    ) -> EpiRstJsonReply {
        Ok(self
            .get(&format!(
                "module/{}/{}/{}/{}/{}/registered",
                year, module, instance, activity, event
            ))
            .await?)
    }

    pub async fn get_internship(&self) -> EpiRstJsonReply {
        Ok(self.get("stage").await?)
    }

    pub async fn get_autologin(&self) -> EpiRstJsonReply {
        Ok(self.get("admin/autologin").await?)
    }

    /*
    pub async fn trombi(
        &self,
        location: &Vec<String>,
        year: &Vec<String>,
        prom: &Vec<String>,
    ) -> EpiRstJsonReply {
        Err(std::io::Error::new(std::io::ErrorKind::Other, "Not implemented yet").into())
    }
    */

    pub fn new(token: &str) -> EpiRst {
        EpiRst {
            autologin_token: token.to_string(),
            client: Client::new(),
        }
    }
}
