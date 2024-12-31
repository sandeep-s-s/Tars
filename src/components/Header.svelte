<script>
    import arya_img from "../image/arya.png";
</script>


<nav class="navbar navbar-expand-sm navbar-light bg-light border-bottom">
	<div class="container-fluid">
        <!-- <img src="{arya_img}" alt="arya logo" /> -->
		<!-- <a class="navbar-brand" href="javascript:void(0)">Arya</a> -->
        <a class="navbar-brand" href="#">
            <img src="{arya_img}" alt="Avatar Logo" style="width:80px;" class="rounded-pill"> 
          </a>
		<button
			class="navbar-toggler"
			type="button"
			data-bs-toggle="collapse"
			data-bs-target="#mynavbar"
		>
			<span class="navbar-toggler-icon"></span>
		</button>
		<div class="collapse navbar-collapse" id="mynavbar">
			<ul class="navbar-nav me-auto">
				<li class="nav-item">
					<a class="nav-link" href="javascript:void(0)">Link</a>
				</li>
			</ul>
			<div class="d-flex">
				<ul class="navbar-nav">
					<li class="nav-item">
						<a class="nav-link" href="javascript:void(0)"><i class="bi bi-gear-fill"></i> Settings</a>
					</li>
				</ul>
			</div>
		</div>
	</div>
</nav>

