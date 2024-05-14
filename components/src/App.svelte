<script lang="ts">
  import 'htmx.org'

  export let current;
  export let other;
</script>

<style>
    /* CSS style for the container of grid items */
    .GridContainer {
      display: grid;
      grid-template-columns: repeat(2, 1fr); /* Two equal-width columns */
      grid-gap: 0px; /* Gap between grid items */
      margin-top: 20px; /* Add margin at the top */
    }
  
    /* CSS style for the square container */
    .SquareContainer {
      width: 400px; /* Set a fixed width */
      height: 200px; /* Set a fixed height to make it square */
      background-color: #FFFFFF; /* White background */
      border: 2px solid #DDDDDD;
      border-radius: 6px;
      color: #3B3C3E;
      font-size: 12px;
      padding: 10px; /* Add some padding inside the container */
      box-sizing: border-box; /* Include padding and border in width */
      overflow: auto; /* Enable scrolling for content overflow */
    }
  
    /* CSS style for the container header */
    .BigHeader {
      text-align: left; /* Left-align the text */
      margin-bottom: 20px; /* Add some space below the header */
      font-size: 20px; /* Increase font size for big header */
          margin-left: 27px	
      }
  
    /* CSS style for the container wrapper */
    .ContainerWrapper {
      display: flex;
      flex-direction: column;
      align-items: center;
    }
  
    /* CSS style for the text input */
    .TextInput {
      width: 400px;    
      height: 25px;    
      margin-bottom: 10px; /* Add space below the text input */
      background-color: #FFFFFF; /* White background */
      border: 2px solid #DDDDDD;
      border-radius: 6px;
      color: #3B3C3E;
    }
  
    /* CSS style for h2 headers */
    h2 {
        font-family: 'Quicksand', sans-serif; /* Specify the custom font */
        color: #595756
    }

    /* Adjust margin or padding for the second column */
    .SecondColumn {
      margin-top: 1px; /* Add margin at the top */
    }
  
    /* CSS style for the button */
    .ShuffleButton {
      width: 150px; /* Set a fixed width */
      height: 40px; /* Set a fixed height */
      background-color: #4CAF50; /* Green background color */
      color: white; /* White text color */
      border: none; /* Remove border */
      border-radius: 6px; /* Add rounded corners */
      font-size: 16px; /* Font size */
      cursor: pointer; /* Add cursor on hover */
    }

    .ShuffleButton:hover {
      background-color: #45a049; /* Darker green on hover */
    }
  </style>
  
  <!-- HTML code for the text input and the two columns of scrollable square containers with headers -->
  <div class="BigHeader">User: {current}</div>
  
  <div class="GridContainer">
    <div class="ContainerWrapper">
      <input type="text" class="TextInput" placeholder="I need...">
      <h2 class="ContainerHeader">My Needs</h2>
      <div class="SquareContainer">
        <!-- Content inside Square Container 1 -->
      </div>
    </div>
  
    <!-- Column 2 -->
    <div class="ContainerWrapper SecondColumn">
      <button class="ShuffleButton">Shuffle Kin</button> <!-- Add button here -->
      <h2 class="ContainerHeader">{other}'s Needs</h2>
      <div class="SquareContainer">
        <!-- Content inside Square Container 2 -->
      </div>
    </div>
  
    <!-- Repeat for Column 3 -->
    <div class="ContainerWrapper">
      <h2 class="ContainerHeader">Notifications</h2>
      <div class="SquareContainer">
        <!-- Content inside Square Container 3 -->
      </div>
    </div>
  
    <!-- Repeat for Column 4 -->
    <div class="ContainerWrapper">
      <h2 class="ContainerHeader">Ledger</h2>
      <div class="SquareContainer">
        <!-- Content inside Square Container 4 -->
      </div>
    </div>
  </div>