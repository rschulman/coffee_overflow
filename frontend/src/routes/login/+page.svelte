<script lang="ts">
	import Button from '@smui/button';
	import Card from '@smui/card';
	import Textfield from '@smui/textfield';
	import { STRINGS } from '$lib/constants/strings';
	import { login, getUserDetails } from '$lib/network/api';
	import { updateUser, type StateHours } from '$lib/stores/user';
	import { goto } from '$app/navigation';
	import { browser } from '$app/environment';

	let username = $state('');
	let password = $state('');
	let loading = $state(false);
	let error = $state('');

	async function handleLogin(e: Event) {
		e.preventDefault();
		loading = true;
		error = '';

		try {
			const response = await login({ username, password });

			// Store token in sessionStorage
			if (browser && response.token) {
				sessionStorage.setItem('token', response.token);
			}

			// Fetch user details (only returns states from backend)
			const userDetails = await getUserDetails();

			// Transform backend response to frontend StateHours format
			const stateHours: StateHours[] = userDetails.states.map(state => ({
				state: state.state_code,
				hoursCompleted: state.hours_complete,
				renewalDate: '' // Backend doesn't store renewal date yet, using empty string as placeholder
			}));

			// Update user store - backend doesn't return username/fullName, so we only have what was entered
			// Note: fullName won't be available on login, only username
			updateUser({
				username,
				fullName: '', // Backend doesn't provide this on login
				stateHours
			});

			// Navigate to dashboard
			goto('/dashboard');
		} catch (err) {
			error = 'Login failed. Please check your credentials.';
			console.error('Login error:', err);
		} finally {
			loading = false;
		}
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

			<form onsubmit={handleLogin} aria-label={STRINGS.aria.signInForm}>
				<div class="form-field">
					<Textfield
						bind:value={username}
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

				{#if error}
					<div class="error-message" role="alert">
						{error}
					</div>
				{/if}

				<Button
					variant="raised"
					class="submit-button"
					type="submit"
					disabled={loading}
					aria-label={STRINGS.aria.signInToAccount}
				>
					<span style="font-weight: 600;">{loading ? 'Signing in...' : STRINGS.login.signIn}</span>
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

	.error-message {
		background: #fef2f2;
		border: 1px solid #fecaca;
		color: #dc2626;
		padding: 0.75rem 1rem;
		border-radius: 4px;
		margin-bottom: 1rem;
		font-size: 0.875rem;
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
