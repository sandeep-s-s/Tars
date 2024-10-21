<script>
	import RequestInput from './request/RequestInput.svelte';
	import RequestNav from './request/RequestNav.svelte';
	let request = {
		v: '1',
		endpoint: 'https://echo.hoppscotch.io',
		name: 'Get-req',
		params: [
			{
				key: 'name',
				value: 'sanu',
				checked: true
			},
			{
				key: 'age',
				value: '28',
				checked: true
			},
			{
				key: 'address',
				value: 'sanu nivas',
				checked: false
			}
		],
		headers: [
			{
				key: 'header1',
				value: 'header1value',
				checked: true
			},
			{
				key: 'header2',
				value: 'header2value',
				checked: false
			}
		],
		method: 'GET',
		auth: {
			authType: 'basic',
			authActive: true,
			username: 'hello',
			password: 'letmein',
			token:"abcdefgh"
		},
		preRequestScript: '',
		testScript: '',
		body: {
			contentType: null,
			body: null
		}
	};
</script>

<div class="col">
<!-- <pre>{JSON.stringify(request, null, 1)}</pre> -->
	<RequestInput {request} bind:method={request.method} bind:endpoint={request.endpoint} />
	<RequestNav bind:request={request} />
</div>
