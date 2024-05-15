<style>
    li {
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

    span {
        flex: 1;
        font-family: -apple-system, system-ui, "Segoe UI", Helvetica, Arial, sans-serif, "Apple Color Emoji", "Segoe UI Emoji";
        font-size: 14px;
        font-weight: 500; /* Let the text expand to fill available space */
        padding: 6px 16px;
        color: #595756;
    }

    /* CSS style for buttons */
    button {
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

<li>
    <span><slot /></span>
    <button>Duplicate</button>
    <button>Delete</button>
</li>
