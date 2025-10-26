<script lang="ts">
	import Button from '@smui/button';
	import Card from '@smui/card';
	import Textfield from '@smui/textfield';
	import { goto } from '$app/navigation';
	import { STRINGS } from '$lib/constants/strings';

	let email = $state('');
	let password = $state('');

	function handleLogin() {
		// Simple navigation to dashboard for demo purposes
		goto('/dashboard');
	}
</script>

<main class="login-container" aria-label={STRINGS.aria.loginPage}>
	<section class="login-card">
		<header class="logo-section">
			<h1 class="app-title">{STRINGS.appName}</h1>
			<p class="tagline">{STRINGS.appTagline}</p>
		</header>

		<Card style="padding: 2rem; width: 100%; max-width: 400px;">
			<h2 class="card-title">{STRINGS.login.welcomeBack}</h2>

			<form on:submit|preventDefault={handleLogin} aria-label={STRINGS.aria.signInForm}>
				<div class="form-field">
					<Textfield
						bind:value={email}
						label={STRINGS.login.email}
						type="email"
						style="width: 100%;"
						variant="outlined"
						required
						input$aria-label={STRINGS.login.emailLabel}
					/>
				</div>

				<div class="form-field">
					<Textfield
						bind:value={password}
						label={STRINGS.login.password}
						type="password"
						style="width: 100%;"
						variant="outlined"
						required
						input$aria-label={STRINGS.login.passwordLabel}
					/>
				</div>

				<Button
					variant="raised"
					class="submit-button"
					type="submit"
					aria-label={STRINGS.aria.signInToAccount}
				>
					<span style="font-weight: 600;">{STRINGS.login.signIn}</span>
				</Button>
			</form>

			<p class="signup-link">
				{STRINGS.login.dontHaveAccount} <a href="/">{STRINGS.login.getStarted}</a>
			</p>
		</Card>
	</section>
</main>

<style>
	.login-container {
		min-height: 100vh;
		display: flex;
		align-items: center;
		justify-content: center;
		background: linear-gradient(135deg, #4f46e5 0%, #581c87 100%);
		padding: 1rem;
	}

	.login-card {
		display: flex;
		flex-direction: column;
		align-items: center;
		gap: 2rem;
		max-width: 500px;
		width: 100%;
	}

	.logo-section {
		text-align: center;
		color: white;
	}

	.app-title {
		font-size: 3rem;
		font-weight: 700;
		margin-bottom: 0.5rem;
		background: linear-gradient(135deg, #fff 0%, #e0e7ff 100%);
		-webkit-background-clip: text;
		-webkit-text-fill-color: transparent;
		background-clip: text;
	}

	.tagline {
		font-size: 1.1rem;
		color: #f3f4f6;
		font-weight: 300;
	}

	:global(.mdc-card) {
		background: white !important;
		box-shadow: 0 20px 25px -5px rgba(0, 0, 0, 0.3), 0 10px 10px -5px rgba(0, 0, 0, 0.2) !important;
	}

	.card-title {
		font-size: 1.5rem;
		font-weight: 600;
		margin-bottom: 1.5rem;
		color: #1f2937;
	}

	.form-field {
		margin-bottom: 1.25rem;
	}

	:global(.mdc-text-field__input),
	:global(.mdc-floating-label) {
		color: #1f2937 !important;
	}

	:global(.submit-button),
	:global(.submit-button::before),
	:global(.submit-button .mdc-button__ripple) {
		width: 100%;
		margin-top: 1rem;
		background: linear-gradient(135deg, #3B82F6 0%, #A855F7 100%) !important;
		color: white !important;
		min-height: 44px;
	}

	:global(.submit-button) {
		border-radius: 4px;
		overflow: hidden;
	}

	:global(.submit-button:focus) {
		outline: 3px solid #fff;
		outline-offset: 2px;
	}

	:global(.mdc-text-field) {
		width: 100% !important;
	}

	:global(.mdc-text-field:focus-within) {
		outline: 2px solid #3B82F6;
		outline-offset: 2px;
		border-radius: 4px;
	}

	.signup-link {
		text-align: center;
		margin-top: 1rem;
		font-size: 0.9rem;
		color: #6b7280;
	}

	.signup-link a {
		color: #3B82F6;
		text-decoration: none;
		font-weight: 600;
		display: inline-block;
		min-height: 44px;
		line-height: 44px;
	}

	.signup-link a:hover {
		text-decoration: underline;
	}

	.signup-link a:focus {
		outline: 2px solid #3B82F6;
		outline-offset: 2px;
		border-radius: 2px;
	}

	/* Mobile responsive styles */
	@media (max-width: 640px) {
		.app-title {
			font-size: 2rem;
		}

		.tagline {
			font-size: 0.95rem;
		}

		.login-card {
			gap: 1.5rem;
		}

		.card-title {
			font-size: 1.25rem;
		}
	}

	@media (max-width: 400px) {
		.login-container {
			padding: 0.5rem;
		}

		.app-title {
			font-size: 1.75rem;
		}

		:global(.mdc-card) {
			padding: 1.5rem !important;
		}
	}
</style>
