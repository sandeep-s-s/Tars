<script>
    export let request = {}
    let methods = [
        { id: 1, text: 'GET', default: true },
		{ id: 2, text: 'POST', default:false },
		{ id: 3, text: 'PUT', default:false }
    ]

    export let method = request["method"]
    export let endpoint = request["endpoint"]
    async function sendPostRequest() {
    try {
      const response = await fetch('your_api_endpoint', {
        method: 'POST',
        headers: {
          'Content-Type': 'application/json'
        },
        body: JSON.stringify(request)
      });
      
      const responseData = await response.json();
      responseMessage = responseData.message; // Assuming API sends back a message
    } catch (error) {
      console.error('Error:', error);
      responseMessage = "Error occurred while sending request.";
    }
  } 
</script>
<!-- <pre>{JSON.stringify(request, null, 1)}</pre> -->
<form on:submit|preventDefault={sendPostRequest}>
    <div class="request-top-container">
        <div class="request-select">
            <select bind:value={method} class="form-control">
                {#each methods as method}
                    <option value={method.text}>
                        {method.text}
                    </option>
                {/each}
            </select>
        </div>
        <div class="request-url">
            <input bind:value={endpoint} class={'form-control'} placeholder="URL"/>
        </div>
        <div class="request-button">
            <button type="submit" class='btn btn-outline-danger'>Send</button>
        </div>
    </div>
</form>

<style>
  .request-top-container{
      display: flex;
      gap: 30px;
  }
</style>