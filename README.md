# Password Generator

A web app where users can specify password criteria (e.g., length, inclusion of symbols/numbers), and the app generates a secure password.

Not very sophisticated, just wanted to try using WASM with Rust.

![Screenshot 2025-01-08 220715](https://github.com/user-attachments/assets/b7508f98-a4fe-401b-81f1-50a6632dd116)

## Instructions for use
1. Run the command `wasm-pack build --target bundler`
1. Navigate to the site directory `cd site`
2. Run the command `npm i` to install dependancies
3. Run the command `npm link ../pkg/` to link to the password generator package
4. Run the command `npm run dev` and enjoy!
