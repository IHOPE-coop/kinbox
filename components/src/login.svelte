<script>
    let username = ''; // This variable will hold the value entered in the username text box
    let password = ''; // This variable will hold the value entered in the password text box
  </script>
  
  <style>
    /* CSS style for the square container */
    .SquareContainer {
      width: 400px; /* Set a fixed width */
      height: 250px; /* Adjusted height to accommodate the buttons */
      background-color: #FFFFFF; /* White background */
      border: 2px solid #DDDDDD;
      border-radius: 6px;
      color: #3B3C3E;
      font-size: 12px;
      padding: 10px; /* Add some padding inside the container */
      box-sizing: border-box; /* Include padding and border in width */
    }
  
    /* CSS style for the container wrapper */
    .ContainerWrapper {
      display: flex;
      flex-direction: column;
      align-items: center;
    }
  
    /* CSS style for the text input */
    .TextInput {
      width: 100%;
      height: 25px;
      margin-bottom: 10px; /* Add space below the text input */
      background-color: #FFFFFF; /* White background */
      border: 2px solid #DDDDDD;
      border-radius: 6px;
      color: #3B3C3E;
      box-sizing: border-box; /* Include padding and border in width */
    }
  
    /* CSS style for h2 headers */
    h2 {
      font-family: 'Quicksand', sans-serif; /* Specify the custom font */
      color: #595756;
      margin-bottom: 10px; /* Add margin below the header */
    }
  
    /* CSS style for buttons */
    .log-in-button,
    .new-user-button,
    .forgot-password-button {
      width: 100%;
      margin-top: 10px; /* Add margin above the buttons */
      padding: 8px 16px; /* Adjusted padding for buttons */
      background-color: #FAFBFC;
      border: 2px solid rgba(27, 31, 35, 0.15);
      border-radius: 6px;
      color: #595756;
      cursor: pointer;
      font-family: -apple-system, system-ui, "Segoe UI", Helvetica, Arial, sans-serif, "Apple Color Emoji", "Segoe UI Emoji";
      font-size: 14px;
      font-weight: 500;
      line-height: 20px;
      text-align: center;
      transition: background-color 0.2s cubic-bezier(0.3, 0, 0.5, 1);
      -webkit-user-select: none;
      touch-action: manipulation;
      white-space: nowrap;
      word-wrap: break-word;
    }
  </style>
  
  <!-- HTML code for the sign-in form -->
  <div class="ContainerWrapper">
    <h2 class="ContainerHeader">Sign In</h2>
    <div class="SquareContainer">
      <form>
        <input type="text" class="TextInput" placeholder="Username" value="{username}">
        <input type="password" class="TextInput" placeholder="Password" value="{password}">
        <button type="submit" class="log-in-button">Log In</button>
      </form>
      <button class="new-user-button">New User</button>
      <button class="forgot-password-button">Forgot Password</button>
    </div>
  </div>