<script>
    import { createEventDispatcher, onMount } from 'svelte';
    let dispatch = createEventDispatcher();

    const closePopup = () => {
        dispatch("close");
    };

    const reloadPage = () => {
        dispatch("submit");
    };

    const rgbToHex = (rgbStr) => {
        const result = rgbStr.match(/\d+/g);
        return parseInt(`${(parseInt(result[0]) << 16) | (parseInt(result[1]) << 8) | parseInt(result[2])}`).toString(16);
    }

    export let deleteByName;

    export let isEditing;
    export let editedElement;

    onMount(async () => {

        if(!isEditing) {
            // do NOT use editedElement here fyi... its not for this bit, its for the else.
            console.log("Starting fresh form..");

            let colours = "";
            let br = Math.floor(Math.random() * 100 + 50); // 100->150
            let bg = Math.floor(Math.random() * 100 + 50);
            let bb = Math.floor(Math.random() * 100 + 50);
            for(let i = 0; i < 6; i++) {
                colours += rgbToHex(`rgb(${br + (i * 15)}, ${bg + (i * 15)}, ${bb + (i * 15)})`) + (i < (6 - 1) ? "," : "");
            }
            document.getElementsByName("colours")[0].value = colours;
        } else {

            let colours = [];
            Array.from(editedElement.children[0].children).forEach(item => { colours.push(rgbToHex(window.getComputedStyle(item).backgroundColor)); });

            document.getElementsByName("old_name")[0].value = editedElement.children[1].firstChild.textContent;
            document.getElementsByName("name")[0].value = editedElement.children[1].firstChild.textContent;
            document.getElementsByName("colours")[0].value = colours.join(",");
            let recipeInformationElement = editedElement.children[1].children[2];
            document.getElementsByName("grind_size")[0].value = recipeInformationElement.children[0].children[1].textContent;
            document.getElementsByName("temperature")[0].value = recipeInformationElement.children[1].children[1].textContent.substring(1); // remove the tilde
            document.getElementsByName("shot_volume")[0].value = recipeInformationElement.children[2].children[1].textContent;
            document.getElementsByName("shot_time")[0].value = recipeInformationElement.children[3].children[1].textContent;
            document.getElementsByName("shot_weight")[0].value = recipeInformationElement.children[4].children[1].textContent;

        }
    });

</script>


<div class="dark-background" />
<div class="add-popup">
    <iframe name="dummyframe" id="dummyframe" style="display: none;"></iframe>

    <div class="add-title">Add Brew</div>
    <form method="GET" action="/api/add" class="form" target="dummyframe" on:submit={reloadPage}>
        <input class="input" type="hidden" name="old_name">
        <label>Name<input class="input" name="name" type="text" style="width: 200px !important"></label>
        <label>Colours<input class="input" name="colours" type="text" style="width: 200px !important"></label>
        <label>Grind Size<input class="input" name="grind_size" type="number" min="1" max="40"></label>
        <label>Brew Temperature <input class="input" name="temperature" type="number" step="0.1" min="1" max="120"></label>
        <label>Shot Volume<input class="input" name="shot_volume" type="number" min="1" max="120"></label>
        <label>Shot Time<input class="input" name="shot_time" type="number" min="1" max="120"></label>
        <label>Shot Weight<input class="input" name="shot_weight" type="number" min="12" max="24" step="0.1"></label>
        <div class="form-submission">
            <button type="submit" class="submit-form">
                {#if isEditing}
                    Update
                {:else}
                    Add
                {/if}
            </button>
            {#if isEditing}
                <button on:click|preventDefault={() => { deleteByName(editedElement.children[1].firstChild.textContent); }} class="submit-form">Delete</button>
            {/if}
            <button on:click={closePopup} class="submit-form">Close</button>
        </div>
    </form>
</div>

