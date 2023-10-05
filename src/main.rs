
mod request_handler;

use tokio;
use request_handler::{AccessToken, create_custom_agent, delete_custom_agent};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Load environment variables from .env file
    dotenv::dotenv().ok();
    let mut access_token = AccessToken::new(24 * 3600);

   let input = vec![
      String::from("0x6f40DF8cC60F52125467838D15f9080748c2baea"),
      0.to_string()
   ];
   let threshold: u32 = 10062912;
   let discord = "https://discord.com/api/webhooks/1234/slack".to_string();
   let email: Option<String> = None; 

   let response = create_custom_agent(&mut access_token, input, threshold, email, Some(discord)).await?;
   match response {
      Some(data) => {
         println!("Agent {:#?} has been created", &data.agent_id());
         delete_custom_agent(&mut access_token, &data.agent_id()).await?
      }
      None => {
         println!("Alert already exists!");
      }
   }

   Ok(())
}




// let body = r#"{
//    "agentType": "genericNodeQuery",
//    "agentName": "COOLER API TEST",
//    "severity": "Medium",
//    "muteDuration": 0,
//    "state": "enabled",
//    "rule": {
//       "chain": "ethereum",
//       "input": [
//          "0x6f40DF8cC60F52125467838D15f9080748c2baea",
//          "0"
//       ],
//       "funcSig": "timeToExpiry(address cooler_, uint256 id_)",
//       "fileName": "",
//       "operands": [
//          "10062912"
//       ],
//       "operator": "lt",
//       "ruleString": "On Ethereum: when Cooler Loan expires in less than 10062912 secs",
//       "outputIndex": 0,
//       "inputDataType": [
//          "address",
//          "uint256"
//       ],
//       "outputDataType": [
//          "uint256"
//       ],
//       "contractAddress": "0xA00F4b7c57a4995796D6E2ae4A6D5dEc8a557367",
//       "contractAddressAlias": "Cooler Monitoring",
//       "contractFunctionObject": {
//          "name": "timeToExpiry",
//          "type": "function",
//          "inputs": [
//             {
//                "name": "cooler_",
//                "type": "address"
//             },
//             {
//                "name": "id_",
//                "type": "uint256"
//             }
//          ],
//          "outputs": [
//             {
//                "name": "timeLeft",
//                "type": "uint256"
//             }
//          ],
//          "stateMutability": "view"
//       }
//    },
//    "channelsConfigurations": [
//       {
//          "channelType": "Email",
//          "configuration": {
//             "email": [
//                "0xrusowsky@gmail.com"
//             ]
//          }
//       },
//       {
//          "channelType": "Discord",
//          "configuration": {
//             "url": [
//                "https://discord.com/api/webhooks/1156145382854766612/cMTzbRaa31-QJoW-uGBTvbKcsxllRXGhMGZWvZOv01iBLfeVSxtpGBM3e44XCgGsVHBc/slack"
//             ]
//          }
//       }
//    ],
//    "remindersConfigurations": [],
//    "delay": 600
// }
// "#;