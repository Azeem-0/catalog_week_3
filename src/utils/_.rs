// let mut cursor = self.col.find(filter, None).await?;

// let mut results = Vec::new();

// while let Some(result) = cursor.next().await {
//     match result {
//         Ok(document) => results.push(document),
//         Err(e) => eprintln!("Error fetching document: {:?}", e),
//     }

//     if results.len() as f64 >= count {
//         break;
//     }
// }

// Ok(results)
