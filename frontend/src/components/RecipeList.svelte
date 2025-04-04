<script>
    import { onMount } from 'svelte';
    import RecipePopup from "./RecipePopup.svelte"

    let recipes = [];
    
    onMount(async () => {
        try {
            const response = await fetch('/api/get');
            if (response.ok) {
                recipes = (await response.json());
            } else {
                console.error('Failed to fetch data');
                // if testing...
                // recipes = JSON.parse(`[{"name":"Waitrose Peru Decaffinated","colours":"303030,3552b5,e3e3e3","grind_size":15,"shot_volume":36,"shot_time":20,"temperature":95.0,"shot_weight":18.0},{"name":"Taylors of Harrogate Italian","colours":"3a3a3a,405d46,85ac73,dfdde3","grind_size":18,"shot_volume":36,"shot_time":22,"temperature":95.0,"shot_weight":18.0},{"name":"Tesco Colombian Decaf","colours":"dd3925,f69b3c,e1e0e0","grind_size":14,"shot_volume":33,"shot_time":20,"temperature":95.0,"shot_weight":18.0},{"name":"Costa Amazonian Blend","colours":"7d2036,fe714e,5fd5c7,b062b0","grind_size":17,"shot_volume":36,"shot_time":20,"temperature":95.0,"shot_weight":18.0},{"name":"Löfbergs Brazil","colours":"381147,4F375B,f8c327","grind_size":18,"shot_volume":36,"shot_time":20,"temperature":95.0,"shot_weight":18.0},{"name":"Ueshima Fuji Mountain","colours":"292929,ef2720,eaeaea","grind_size":17,"shot_volume":18,"shot_time":33,"temperature":95.0,"shot_weight":18.0},{"name":"Taylors of Harrogate Decaf","colours":"174A84,309CCB,76C9EF,F6F5F0","grind_size":17,"shot_volume":36,"shot_time":18,"temperature":95.0,"shot_weight":17.8}]`);
            }
        } catch (error) {
            console.error('Error fetching data:', error);
        }
    });

    let is_adding = false;
    let is_editing = false;
    let pressed_element = undefined;

    const openPopup = (edit, element) => {
        is_editing = edit;
        if(edit) {
            pressed_element = element.closest(".recipe-container");
        }
        is_adding = true;
    };

    const closePopup = () => {
        is_adding = false;
    };

    const reloadPage = () => {
        location.reload();
    };

    const removeRecipe = async (name) => {
        const response = await fetch("/api/remove?name=" + name);
        if(response.ok) {
            location.reload();
        }
    }
</script>


<div class="app-container">
    <div class="all-recipes-container">

        {#each recipes as item}
            <div class="recipe-container">
                <div class="colour-container">
                    {#each item.colours.split(",") as hex_col, index}
                        {#if index == 0}
                            <div class="colour-block" style="width: 100% !important; background-color: #{hex_col}; border-top-left-radius: 5px;"></div>
                        {:else if index == item.colours.split(",").length-1}
                            <div class="colour-block" style="width: 50%; background-color: #{hex_col}; border-top-right-radius: 5px;"></div>
                        {:else}
                            <div class="colour-block" style="background-color: #{hex_col};"></div>
                        {/if}
                    {/each}
                </div>
                <div class="recipe-information">
                    <p class="recipe-title">{item.name}</p>
                    <div class="line-break"></div>
                    <div class="stat-container">
                        <div class="single-stat"><p class="feature-key">Grind size: </p><p class="feature-value"> {item.grind_size}</p></div>
                        <div class="single-stat"><p class="feature-key">Temperature: </p><span class="feature-value"> ~{item.temperature}</span><span class="feature-value-unit">°c</span></div>
                        <div class="single-stat"><p class="feature-key">Shot volume: </p><span class="feature-value"> {item.shot_volume}</span><span class="feature-value-unit">ml</span></div>
                        <div class="single-stat"><p class="feature-key">Shot time: </p><span class="feature-value"> {item.shot_time}</span><span class="feature-value-unit">s</span></div>
                        <div class="single-stat"><p class="feature-key">Shot weight: </p><span class="feature-value"> {parseFloat(item.shot_weight).toFixed(1)}</span><span class="feature-value-unit">g</span></div>
                    </div>
                    <div on:click={(event) => { openPopup(true, event.target); }} class="remove">
                        <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24"><g fill="none" stroke="currentColor" stroke-linecap="round" stroke-linejoin="round" stroke-width="1.6"><path d="M7 7H6a2 2 0 0 0-2 2v9a2 2 0 0 0 2 2h9a2 2 0 0 0 2-2v-1"/><path d="M20.385 6.585a2.1 2.1 0 0 0-2.97-2.97L9 12v3h3zM16 5l3 3"/></g></svg>
                    </div>
                </div>
            </div>
        {/each}
        <div on:click={() => { is_editing = false; openPopup(); }} class="recipe-container override-addition">
            <svg class="add-icon" xmlns="http://www.w3.org/2000/svg" width="2em" height="2em" viewBox="0 0 24 24" {...$$props}>
	            <path fill="currentColor" d="M11 17h2v-4h4v-2h-4V7h-2v4H7v2h4zm1 5q-2.075 0-3.9-.788t-3.175-2.137T2.788 15.9T2 12t.788-3.9t2.137-3.175T8.1 2.788T12 2t3.9.788t3.175 2.137T21.213 8.1T22 12t-.788 3.9t-2.137 3.175t-3.175 2.138T12 22m0-2q3.35 0 5.675-2.325T20 12t-2.325-5.675T12 4T6.325 6.325T4 12t2.325 5.675T12 20m0-8" />
            </svg>
        </div>

        {#if is_adding}
            <RecipePopup on:close={closePopup} on:submit={reloadPage} isEditing={is_editing} editedElement={pressed_element} deleteByName={removeRecipe} />
        {/if}
    </div>
</div>
