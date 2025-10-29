<script lang="ts">
	import Card from '@smui/card';
	import Textfield from '@smui/textfield';
	import Button from '@smui/button';
	import { goto } from '$app/navigation';
	import { updateUser, type StateHours } from '$lib/stores/user';
	import { State } from '$lib/network/types';
	import { STRINGS } from '$lib/constants/strings';
	import GradientButton from '$lib/components/shared/GradientButton.svelte';
	import FormSection from '$lib/components/onboarding/FormSection.svelte';
	import { register, login } from '$lib/network/api';
	import { browser } from '$app/environment';

	// Generate state options from enum
	const stateOptions = Object.entries(State).map(([key, value]) => ({
		code: key,
		name: value
	}));

	let username = $state('');
	let password = $state('');
	let fullName = $state('');
	let stateHours = $state<StateHours[]>([]);
	let currentState = $state('');
	let currentHours = $state('');
	let currentRenewalDate = $state('');
	let inputKey = $state(0); // Key to force re-render of inputs
	let loading = $state(false);
	let error = $state('');

	function addStateHours() {
		if (currentState && currentHours && currentRenewalDate) {
			// Check if state already exists
			const existingIndex = stateHours.findIndex(sh => sh.state === currentState);
			if (existingIndex >= 0) {
				// Update existing state
				stateHours[existingIndex] = { state: currentState, hoursCompleted: Number(currentHours), renewalDate: currentRenewalDate };
				stateHours = [...stateHours];
			} else {
				// Add new state
				stateHours = [...stateHours, { state: currentState, hoursCompleted: Number(currentHours), renewalDate: currentRenewalDate }];
			}
			currentState = '';
			currentHours = '';
			currentRenewalDate = '';
			inputKey++; // Force re-render to reset label positions
		}
	}

	function getStateName(stateCode: string): string {
		return State[stateCode as keyof typeof State] || stateCode;
	}

	function removeStateHours(index: number) {
		stateHours = stateHours.filter((_, i) => i !== index);
	}

	async function handleSubmit(e: Event) {
		e.preventDefault();
		loading = true;
		error = '';

		try {
			// Transform stateHours array into HashMap format that backend expects
			const states: Record<string, { completed: number; due: string }> = {};
			for (const sh of stateHours) {
				states[sh.state] = {
					completed: sh.hoursCompleted,
					due: sh.renewalDate // Already in YYYY-MM-DD format
				};
			}

			// Register the user
			await register({
				username,
				password,
				fullname: fullName, // Backend expects lowercase 'fullname'
				states
			});

			// Automatically log the user in after registration
			const loginResponse = await login({ username, password });

			// Store token in sessionStorage
			if (browser && loginResponse.token) {
				sessionStorage.setItem('token', loginResponse.token);
			}

			// Update user store with user data
			updateUser({
				username,
				fullName,
				stateHours
			});

			// Navigate to dashboard
			goto('/dashboard');
		} catch (err) {
			error = 'Registration failed. Please try again.';
			console.error('Registration error:', err);
		} finally {
			loading = false;
		}
	}
</script>

<main class="onboarding-container" aria-label={STRINGS.aria.onboardingPage}>
	<nav class="top-nav" aria-label={STRINGS.aria.mainNav}>
		<a href="/login" class="login-link" aria-label={STRINGS.aria.alreadyHaveAccountSignIn}>
			{STRINGS.nav.alreadyHaveAccount} <strong>{STRINGS.nav.signIn}</strong>
		</a>
	</nav>

	<section class="onboarding-content">
		<header class="header-section">
			<h1 class="page-title">{STRINGS.onboarding.title}</h1>
			<p class="subtitle">{STRINGS.onboarding.subtitle}</p>
		</header>

		<Card style="padding: 2.5rem; width: 100%; max-width: 700px;">
			<form onsubmit={handleSubmit} aria-label={STRINGS.aria.onboardingForm}>

				<!-- Section 1: Basic Information -->
				<FormSection stepNumber={1} title={STRINGS.onboarding.section1Title}>
					<div class="form-row two-col">
						<div class="form-field">
							<Textfield
								bind:value={username}
								label="Email"
								type="email"
								style="width: 100%;"
								variant="outlined"
								required
								input$aria-label="Your email address"
							/>
						</div>

						<div class="form-field">
							<Textfield
								bind:value={password}
								label="Password"
								type="password"
								style="width: 100%;"
								variant="outlined"
								required
								input$aria-label="Your password"
							/>
						</div>
					</div>

					<div class="form-row">
						<div class="form-field">
							<Textfield
								bind:value={fullName}
								label={STRINGS.onboarding.fullName}
								type="text"
								style="width: 100%;"
								variant="outlined"
								required
								input$aria-label={STRINGS.aria.yourFullName}
							/>
						</div>
					</div>
				</FormSection>

				<div class="section-divider"></div>

				<!-- Section 2: Continuing Education Status -->
				<FormSection stepNumber={2} title={STRINGS.onboarding.section2Title}>
					<p class="section-description">Add each state where you are licensed with CE hours completed and renewal date:</p>

					<div class="state-hours-input">
						<div class="form-row three-col">
							<div class="form-field select-wrapper">
								<select
									id="state-select"
									bind:value={currentState}
									class="custom-select"
									aria-label={STRINGS.aria.selectYourState}
								>
									<option value="" disabled selected hidden>{STRINGS.onboarding.selectState}</option>
									{#each stateOptions as state}
										<option value={state.code}>{state.name}</option>
									{/each}
								</select>
								<label for="state-select" class="floating-label">{STRINGS.onboarding.state}</label>
							</div>

							{#key inputKey}
								<div class="form-field">
									<Textfield
										bind:value={currentHours}
										label={STRINGS.onboarding.hoursCompleted}
										type="number"
										style="width: 100%;"
										variant="outlined"
										input$aria-label={STRINGS.aria.completedCEHours}
									/>
								</div>

								<div class="form-field">
									<Textfield
										bind:value={currentRenewalDate}
										label={STRINGS.onboarding.renewalDate}
										type="date"
										style="width: 100%;"
										variant="outlined"
										input$aria-label={STRINGS.aria.licenseRenewalDate}
									/>
								</div>
							{/key}
						</div>

						<Button
							variant="outlined"
							onclick={addStateHours}
							disabled={!currentState || !currentHours || !currentRenewalDate}
							style="min-height: 44px; margin-top: 0.5rem;"
						>
							<span style="font-weight: 600;">Add State</span>
						</Button>
					</div>

					{#if stateHours.length > 0}
						<div class="state-hours-list">
							<h4 class="list-title">Your States & Hours:</h4>
							{#each stateHours as stateHour, index}
								<div class="state-hours-item">
									<div class="state-hours-info">
										<span class="state-name">{getStateName(stateHour.state)}</span>
										<div class="state-details">
											<span class="hours-value">{stateHour.hoursCompleted} hours completed</span>
											<span class="renewal-value">Renews: {new Date(stateHour.renewalDate).toLocaleDateString()}</span>
										</div>
									</div>
									<button
										type="button"
										class="remove-button"
										onclick={() => removeStateHours(index)}
										aria-label="Remove {getStateName(stateHour.state)}"
									>
										<span class="material-icons">close</span>
									</button>
								</div>
							{/each}
						</div>
					{/if}
				</FormSection>

				<div class="info-box">
					<span class="material-icons" aria-hidden="true">info</span>
					<p>{STRINGS.onboarding.infoText}</p>
				</div>

				{#if error}
					<div class="error-message" role="alert">
						{error}
					</div>
				{/if}

				<div class="button-group">
					<GradientButton type="submit" fullWidth={true} disabled={loading} ariaLabel={STRINGS.aria.completeOnboarding}>
						<span style="font-weight: 600; color: white;">
							{loading ? 'Creating account...' : STRINGS.onboarding.startLearning}
						</span>
					</GradientButton>
				</div>
			</form>
		</Card>
	</section>
</main>

<style>
	.onboarding-container {
		min-height: 100vh;
		display: flex;
		flex-direction: column;
		background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
		padding: 0;
	}

	.top-nav {
		display: flex;
		justify-content: flex-end;
		width: 100%;
	}

	.login-link {
		color: white;
		text-decoration: none;
		font-size: 0.95rem;
		padding: 0.75rem 1.5rem;
		margin: 1rem;
		border-radius: 8px;
		background: rgba(255, 255, 255, 0.1);
		backdrop-filter: blur(10px);
		transition: all 0.2s;
		min-height: 44px;
		display: inline-flex;
		align-items: center;
		gap: 0.5rem;
	}

	.login-link:hover {
		background: rgba(255, 255, 255, 0.2);
	}

	.login-link:focus {
		outline: 2px solid white;
		outline-offset: 2px;
	}

	.login-link strong {
		font-weight: 600;
	}

	.onboarding-content {
		display: flex;
		flex-direction: column;
		align-items: center;
		justify-content: center;
		gap: 2rem;
		max-width: 800px;
		width: 100%;
		margin: 0 auto;
		padding: 2rem 1rem;
		flex: 1;
	}

	.header-section {
		text-align: center;
		color: white;
	}

	.page-title {
		font-size: 2.5rem;
		font-weight: 700;
		margin-bottom: 0.5rem;
		background: linear-gradient(135deg, #fff 0%, #e0e7ff 100%);
		-webkit-background-clip: text;
		-webkit-text-fill-color: transparent;
		background-clip: text;
	}

	.subtitle {
		font-size: 1.1rem;
		color: #f3f4f6;
		font-weight: 300;
	}

	.section-divider {
		height: 1px;
		background: linear-gradient(to right, transparent, #e5e7eb, transparent);
		margin: 2rem 0;
	}

	.section-description {
		font-size: 0.9rem;
		color: #6b7280;
		margin-bottom: 1.5rem;
	}

	.state-hours-input {
		margin-bottom: 1.5rem;
	}

	.state-hours-list {
		margin-top: 1.5rem;
		padding: 1rem;
		background: #f9fafb;
		border-radius: 8px;
	}

	.list-title {
		font-size: 0.95rem;
		font-weight: 600;
		color: #374151;
		margin-bottom: 0.75rem;
	}

	.state-hours-item {
		display: flex;
		align-items: center;
		justify-content: space-between;
		padding: 0.75rem 1rem;
		background: white;
		border: 1px solid #e5e7eb;
		border-radius: 6px;
		margin-bottom: 0.5rem;
		transition: all 0.2s;
	}

	.state-hours-item:hover {
		border-color: #d1d5db;
		box-shadow: 0 1px 3px rgba(0, 0, 0, 0.1);
	}

	.state-hours-info {
		display: flex;
		flex-direction: column;
		gap: 0.5rem;
		flex: 1;
	}

	.state-name {
		font-weight: 600;
		color: #1f2937;
		font-size: 1rem;
	}

	.state-details {
		display: flex;
		flex-direction: column;
		gap: 0.25rem;
	}

	.hours-value,
	.renewal-value {
		font-size: 0.875rem;
		color: #6b7280;
	}

	.renewal-value {
		font-style: italic;
	}

	.remove-button {
		display: flex;
		align-items: center;
		justify-content: center;
		width: 32px;
		height: 32px;
		border: none;
		background: transparent;
		color: #9ca3af;
		cursor: pointer;
		border-radius: 4px;
		transition: all 0.2s;
		padding: 0;
	}

	.remove-button:hover {
		background: #fee2e2;
		color: #dc2626;
	}

	.remove-button:focus {
		outline: 2px solid #3B82F6;
		outline-offset: 2px;
	}

	.remove-button .material-icons {
		font-size: 1.25rem;
	}

	.form-row {
		margin-bottom: 1.5rem;
	}

	.two-col {
		display: grid;
		grid-template-columns: 1fr 1fr;
		gap: 1rem;
	}

	.three-col {
		display: grid;
		grid-template-columns: 1fr 1fr 1fr;
		gap: 1rem;
	}

	.form-field {
		position: relative;
	}

	.select-wrapper {
		position: relative;
	}

	.custom-select {
		width: 100%;
		padding: 1rem 0.875rem;
		font-size: 1rem;
		font-family: Inter, system-ui, sans-serif;
		border: 1px solid rgba(0, 0, 0, 0.38);
		border-radius: 4px;
		background: transparent;
		color: #1f2937;
		cursor: pointer;
		transition: all 0.2s;
		appearance: none;
		background-image: url("data:image/svg+xml,%3Csvg xmlns='http://www.w3.org/2000/svg' width='24' height='24' viewBox='0 0 24 24'%3E%3Cpath fill='%23666' d='M7 10l5 5 5-5z'/%3E%3C/svg%3E");
		background-repeat: no-repeat;
		background-position: right 0.5rem center;
		background-size: 1.5rem;
		padding-right: 2.5rem;
		min-height: 56px;
	}

	.custom-select:hover {
		border-color: rgba(0, 0, 0, 0.87);
	}

	.custom-select:focus {
		outline: none;
		border-color: #3B82F6;
		border-width: 2px;
		padding: calc(1rem - 1px) calc(0.875rem - 1px);
	}

	.custom-select:focus + .floating-label {
		color: #3B82F6;
	}

	.floating-label {
		position: absolute;
		left: 0.875rem;
		top: -0.5rem;
		background: white;
		padding: 0 0.25rem;
		font-size: 0.75rem;
		color: rgba(0, 0, 0, 0.6);
		pointer-events: none;
		transition: all 0.2s;
	}


	.info-box {
		display: flex;
		gap: 0.75rem;
		padding: 1rem;
		background: #eff6ff;
		border-left: 4px solid #3B82F6;
		border-radius: 4px;
		margin-bottom: 1.5rem;
	}

	.info-box .material-icons {
		color: #3B82F6;
		font-size: 1.25rem;
		flex-shrink: 0;
	}

	.info-box p {
		font-size: 0.875rem;
		color: #1e40af;
		margin: 0;
		line-height: 1.5;
	}

	.error-message {
		background: #fef2f2;
		border: 1px solid #fecaca;
		border-left: 4px solid #dc2626;
		color: #dc2626;
		padding: 0.75rem 1rem;
		border-radius: 4px;
		margin-bottom: 1rem;
		font-size: 0.875rem;
	}

	.button-group {
		display: flex;
		gap: 1rem;
		margin-top: 1rem;
	}

	:global(.mdc-text-field) {
		width: 100% !important;
	}

	:global(.mdc-text-field:focus-within) {
		outline: 2px solid #3B82F6;
		outline-offset: 2px;
		border-radius: 4px;
	}

	/* Tablet responsive styles */
	@media (max-width: 1024px) {
		.three-col {
			grid-template-columns: 1fr;
			gap: 1rem;
		}
	}

	/* Mobile responsive styles */
	@media (max-width: 768px) {
		.onboarding-container {
			padding: 1rem 0.5rem;
		}

		.page-title {
			font-size: 1.75rem;
		}

		.subtitle {
			font-size: 0.95rem;
		}

		.two-col,
		.three-col {
			grid-template-columns: 1fr;
			gap: 1.5rem;
		}

		:global(.mdc-card) {
			padding: 1.5rem !important;
		}

		.button-group {
			flex-direction: column;
		}

		.button-group :global(.mdc-button) {
			width: 100%;
		}

		.state-hours-info {
			flex: 1;
		}

		.state-name {
			font-size: 0.9rem;
		}

		.hours-value {
			font-size: 0.8rem;
		}
	}
</style>
