<script lang="ts">
    import 'htmx.org';
    import Need from "$lib/Need.svelte";
    let newItem = ''; // This variable will hold the value entered in the text box
    let myItems: { text: string, id: any }[] = []; // This array will hold the my needs list items
    let notifItems: any = []; // This array will hold the notification list items
    let kinItems: any = []; // This array will hold the kin's needs list items
    let ledgerItems: any = []; // This array will hold the ledger list items

    export let current = "Harley";
    export let other = "Nathan";

    // Function to remove an item from the main list
    function removeItem(itemId: any) {
        myItems = myItems.filter(item => item.id !== itemId); // Filter out the item with the specified ID
    }
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

    /* Adjust margin or padding for the second column */
    .SecondColumn {
        margin-top: 5px; /* Add margin at the top */
    }

    /* CSS style for the list items */
    .list-item {
        display: flex; /* Use flexbox for inline alignment */
        align-items: center; /* Align items vertically */
        justify-content: space-between; /* Space items evenly */
        margin-bottom: 5px; /* Add some vertical spacing between items */
        margin-right: 16px;
    }

    /* CSS style for alternating row colors */
    li:nth-child(even) {
        background-color: #FAFBFC; /* Light gray background for even rows */
    }

    /* CSS style to remove bullet points from list items */
    ul {
        list-style-type: none; /* Remove bullet points */
        padding: 0; /* Remove default padding */
    }

    .item-text {
        flex: 1;
        font-family: -apple-system, system-ui, "Segoe UI", Helvetica, Arial, sans-serif, "Apple Color Emoji", "Segoe UI Emoji";
        font-size: 14px;
        font-weight: 500; /* Let the text expand to fill available space */
        padding: 6px 16px;
        color: #595756;
    }

    /* CSS style for h2 headers */
    h2 {
        font-family: 'Quicksand', sans-serif; /* Specify the custom font */
        color: #595756;
        margin-bottom: 10px; /* Add margin below the header */
    }

    /* CSS style for buttons */
    .duplicate-button,
    .delete-button,
    .shuffle-button {
        margin-left: 10px; /* Add some spacing between buttons */
        align-items: center;
        background: #f5f5fa;
        appearance: none;
        background-color: #FAFBFC;
        border: 2px solid rgba(27, 31, 35, 0.15);
        border-radius: 6px;
        box-shadow: rgba(27, 31, 35, 0.04) 0 1px 0, rgba(255, 255, 255, 0.25) 0 1px 0 inset;
        box-sizing: border-box;
        color: #595756;
        cursor: pointer;
        display: inline-block;
        font-family: -apple-system, system-ui, "Segoe UI", Helvetica, Arial, sans-serif, "Apple Color Emoji", "Segoe UI Emoji";
        font-size: 14px;
        font-weight: 500;
        line-height: 20px;
        padding: 6px 16px;
        transition: background-color 0.2s cubic-bezier(0.3, 0, 0.5, 1);
        -webkit-user-select: none;
        touch-action: manipulation;
        vertical-align: middle;
        white-space: nowrap;
        word-wrap: break-word;
    }
</style>

<!-- HTML code for the text input and the four columns of scrollable square containers with headers -->
<div class="BigHeader">User: {current}</div>

<div class="GridContainer">
    <div class="ContainerWrapper">
        <form hx-post="/hx-needs/{current}" hx-target="#needs" hx-swap="innerHTML">
            <input type="text" name="need" class="TextInput" placeholder="I need...">
        </form>
        <h2 class="ContainerHeader">My Needs</h2>
        <div class="SquareContainer">
            <div class="DivWithScroll" id="needs" hx-get="/hx-needs/{current}" hx-swap="innerHTML" hx-trigger="load">
                I'm supposed to be replaced by htmx
            </div>
        </div>
    </div>

    <!-- Column 2 -->
    <div class="ContainerWrapper SecondColumn">
        <button class="shuffle-button">Shuffle Kin</button> <!-- Add button here -->
        <h2 class="ContainerHeader">{other}'s Needs</h2>
        <div class="SquareContainer">
            <div class="DivWithScroll">
                <ul>
                    {#each kinItems as item}
                        <li class="list-item">
                            <span class="item-text">{item.text}</span>
                            <button class="duplicate-button">Duplicate</button>
                            <button class="delete-button">Delete</button>
                        </li>
                        <!-- Repeat list items as needed -->
                    {/each}
                </ul>
            </div>
        </div>
    </div>

    <!-- Repeat for Column 3 -->
    <div class="ContainerWrapper">
        <h2 class="ContainerHeader">Notifications</h2>
        <div class="SquareContainer">
            <div class="DivWithScroll">
                <ul>
                    {#each notifItems as item}
                        <li class="list-item">
                            <span class="item-text">{item.text}</span>
                            <button class="duplicate-button">Duplicate</button>
                            <button class="delete-button">Delete</button>
                        </li>
                        <!-- Repeat list items as needed -->
                    {/each}
                </ul>
            </div>
        </div>
    </div>

    <!-- Repeat for Column 4 -->
    <div class="ContainerWrapper">
        <h2 class="ContainerHeader">Ledger</h2>
        <div class="SquareContainer">
            <div class="DivWithScroll">
                <ul>
                    {#each ledgerItems as item}
                        <li class="list-item">
                            <span class="item-text">{item.text}</span>
                            <button class="duplicate-button">Duplicate</button>
                            <button class="delete-button">Delete</button>
                        </li>
                        <!-- Repeat list items as needed -->
                    {/each}
                </ul>
            </div>
        </div>
    </div>
</div>
