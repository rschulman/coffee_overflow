<script lang="ts">
	import { STRINGS } from '$lib/constants/strings';
	import { resetUser } from '$lib/stores/user';
	import { goto } from '$app/navigation';
	import { browser } from '$app/environment';

	function handleLogout() {
		// Clear user data from store
		resetUser();

		// Clear session token if it exists
		if (browser) {
			sessionStorage.removeItem('token');
		}

		// Navigate to login page
		goto('/');
	}
</script>

<header class="app-bar">
	<div class="app-bar-content">
		<div class="brand">
			<h1 class="brand-title">{STRINGS.appName}</h1>
		</div>
		<nav class="nav-menu">
			<span class="nav-item active" aria-label={STRINGS.nav.dashboard} aria-current="page">
				<span class="material-icons">dashboard</span>
				<span class="nav-label">{STRINGS.nav.dashboard}</span>
			</span>
			<button class="nav-item logout-button" onclick={handleLogout} aria-label="Log out">
				<span class="material-icons">logout</span>
				<span class="nav-label">Log Out</span>
			</button>
		</nav>
	</div>
</header>

<style>
	.app-bar {
		background: white;
		border-bottom: 1px solid #e5e7eb;
		position: sticky;
		top: 0;
		z-index: 100;
		box-shadow: 0 1px 3px rgba(0, 0, 0, 0.1);
	}

	.app-bar-content {
		max-width: 1400px;
		margin: 0 auto;
		padding: 1rem 2rem;
		display: flex;
		justify-content: space-between;
		align-items: center;
	}

	.brand-title {
		font-size: 1.5rem;
		font-weight: 700;
		background: linear-gradient(135deg, #3B82F6 0%, #A855F7 100%);
		-webkit-background-clip: text;
		-webkit-text-fill-color: transparent;
		background-clip: text;
		margin: 0;
	}

	.nav-menu {
		display: flex;
		gap: 0.5rem;
	}

	.nav-item {
		display: flex;
		align-items: center;
		gap: 0.5rem;
		padding: 0.625rem 1.25rem;
		background: transparent;
		border: none;
		border-radius: 8px;
		color: #6b7280;
		font-size: 0.875rem;
		font-weight: 500;
		cursor: pointer;
		transition: all 0.2s;
		min-height: 44px;
	}

	.nav-item .material-icons {
		font-size: 1.25rem;
	}

	.nav-item:hover {
		background: #f3f4f6;
		color: #1f2937;
	}

	.nav-item:focus {
		outline: 2px solid #3B82F6;
		outline-offset: 2px;
	}

	.nav-item.active {
		background: linear-gradient(135deg, #eff6ff 0%, #f3e8ff 100%);
		color: #3B82F6;
	}

	.logout-button {
		font-family: inherit;
	}

	.logout-button:hover {
		background: #fef2f2;
		color: #dc2626;
	}

	@media (max-width: 768px) {
		.app-bar-content {
			padding: 1rem;
		}

		.nav-label {
			display: none;
		}
	}
</style>
