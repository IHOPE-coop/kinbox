<script lang="ts">
    let items: { text: string; id: number }[] = []; // This array will hold the main list items
    let newItem: string = ''; // This variable will hold the value entered in the text box
    let secondListItems: { text: string; id: number }[] = []; // This array will hold the items from the second list
    let selectedItems: { text: string; id: number }[] = []; // This array will hold the selected items

    // Function to handle adding a new item to the main list
    function addItem() {
        if (newItem.trim() !== '') { // Check if the new item is not empty or whitespace
            items = [...items, { text: newItem, id: Date.now() }]; // Add the new item with a unique ID to the main list
            newItem = ''; // Clear the text box after adding the item
        }
    }

    // Function to toggle selection of an item
    function toggleSelection(id: number) {
        const clickedItem = items.find(item => item.id === id); // Find the item with the specified ID
        if (clickedItem) { // Check if the clickedItem is not undefined
            if (selectedItems.includes(clickedItem)) {
                // If the item is already selected, move it to the second list
                secondListItems = [...secondListItems, clickedItem];
                items = items.filter(item => item.id !== id);
                selectedItems = selectedItems.filter(selected => selected.id !== id);
            } else {
                selectedItems = [...selectedItems, clickedItem]; // Select the item if it's not already selected
            }
        }
    }

    // Function to handle deselecting selected items
    function deselectItems() {
        selectedItems = []; // Clear the selected items array to deselect all items
    }

    // Function to handle hitting the Enter key
    function handleKeyPress(event: KeyboardEvent) {
        if (event.key === 'Enter') {
            addItem(); // Call the addItem function when Enter key is pressed
        }
    }
</script>

<style>
    /* CSS style to remove bullet points from list items */
    ul {
        list-style-type: none; /* Remove bullet points */
        padding: 0; /* Remove default padding */
    }

    /* Style for selected items */
    .selected {
        color: red; /* Change color to red for selected items */
        font-weight: bold; /* Add bold font weight for selected items */
    }
</style>

<!-- HTML code for the text box and main list -->
<input type="text" bind:value={newItem} on:keypress={handleKeyPress} />

<h2>Kinbox</h2>

<ul>
    {#each items as item}
        <li>
            <button class:selected={selectedItems.includes(item)} on:click={() => toggleSelection(item.id)}>
                {item.text}
            </button>
        </li>
    {/each}
</ul>

<!-- Button to deselect selected items -->
<button on:click={deselectItems}>Deselect Items</button>

<hr /> <!-- Horizontal line to separate the two lists -->

<!-- HTML code for the second list -->
<h2>Ledger</h2>
<ul>
    {#each secondListItems as item}
        <li>{item.text}</li>
    {/each}
</ul>