# YewChat 💬
# Muhammad Sean Arsha Galant 2206822963 - Tutorial 10 Part 3
## Reflection

### Experiment 3.1: Original Code
![Screenshot 2024-05-07 103444.png](static%2Fassets%2FScreenshot%202024-05-07%20103444.png)

### Experiment 3.2: Be Creative!
![Screenshot 2024-05-07 120007.png](static%2Fassets%2FScreenshot%202024-05-07%20120007.png)
![Screenshot 2024-05-07 120019.png](static%2Fassets%2FScreenshot%202024-05-07%20120019.png)

I updated the `login.rs` file in the html macro and the `index.html` file. The html! macro in Rust is used to create an HTML structure for a login page with a centered form and an animated gradient background. The outermost <div> centers its child elements horizontally and vertically using flexbox. Inside, there's a semi-transparent, rounded <div> representing the login form container with a maximum width limit.
Within the form container, there's a heading "Welcome Back" styled with a larger font size, bold text, and white color. Below the heading, there's a <form> element with a full-width username <input> field and a "Go Chatting!" <button>. The button is styled with a violet background, white text, rounded corners, and a hover effect. It's also disabled when the username input is empty.
Index.html includes the HTML <!DOCTYPE> declaration, <html> root element, <head> section with metadata and inclusion of the Tailwind CSS library, and a <body> section.
The <body> has a flexbox layout that centers its content horizontally and vertically, with a minimum height equal to the viewport height. It includes a <script> tag for the "yewchat.js" file, which likely contains the application's JavaScript code.
The <style> section defines a CSS animation called "gradient" that creates a smooth transition effect for the background gradient, moving it from left to right and back again over 16 seconds. This animated gradient background is applied to the <body> using inline styles.
