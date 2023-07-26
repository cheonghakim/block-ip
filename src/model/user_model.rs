use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct User {
    id: String,
    password: String,
    email: Option<String>,
}

impl User {
    pub fn new(id: String, password: String, email: Option<String>) -> Self {
        User {
            id,
            password,
            email,
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct LoginForm {
    pub id: String,
    pub password: String,
}

// // Manually implement the Deserialize trait for LoginForm
// impl<'de> Deserialize<'de> for LoginForm {
//     fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
//     where
//         D: serde::Deserializer<'de>,
//     {
//         // Implement the deserialization logic here.
//         // For example, you can use serde's helper methods like `deserialize_struct`
//         // to deserialize the fields of LoginForm.

//         // Use serde's `Visitor` to extract the fields from the input data.
//         // Here we use the `deserialize_struct` method.
//         deserializer.deserialize_struct("LoginForm", &["id", "password"], LoginFormVisitor)
//     }
// }

// // Define a custom Visitor to extract the fields from the input data.
// struct LoginFormVisitor;

// // Implement the Visitor trait for LoginFormVisitor.
// // This defines how the fields should be extracted from the input data.
// impl<'de> serde::de::Visitor<'de> for LoginFormVisitor {
//     type Value = LoginForm;

//     fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
//         formatter.write_str("struct LoginForm")
//     }

//     fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
//     where
//         A: serde::de::SeqAccess<'de>,
//     {
//         // Extract the id and password fields from the input sequence.
//         let id: String = seq
//             .next_element()?
//             .ok_or_else(|| serde::de::Error::invalid_length(0, &self))?;
//         let password: String = seq
//             .next_element()?
//             .ok_or_else(|| serde::de::Error::invalid_length(1, &self))?;

//         // Create and return the LoginForm instance
//         Ok(LoginForm { id, password })
//     }
// }

#[derive(Debug, Deserialize, Serialize)]
pub struct SignupForm {
    pub id: String,
    pub password: String,
    pub email: String,
}

// impl<'de> Deserialize<'de> for SignupForm {
//     fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
//     where
//         D: serde::Deserializer<'de>,
//     {
//         deserializer.deserialize_struct(
//             "SignupForm",
//             &["id", "password", "email"],
//             SignupFormVisitor,
//         )
//     }
// }

// struct SignupFormVisitor;

// impl<'de> serde::de::Visitor<'de> for SignupFormVisitor {
//     type Value = SignupForm;

//     fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
//         formatter.write_str("struct SignupForm")
//     }

//     fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
//     where
//         A: serde::de::SeqAccess<'de>,
//     {
//         let id: String = seq
//             .next_element()?
//             .ok_or_else(|| serde::de::Error::invalid_length(0, &self))?;
//         let password: String = seq
//             .next_element()?
//             .ok_or_else(|| serde::de::Error::invalid_length(1, &self))?;
//         let email: String = seq
//             .next_element()?
//             .ok_or_else(|| serde::de::Error::invalid_length(1, &self))?;
//         // Create and return the LoginForm instance
//         Ok(SignupForm {
//             id,
//             password,
//             email,
//         })
//     }
// }
