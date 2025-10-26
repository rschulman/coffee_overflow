<script lang="ts">
	import { STRINGS } from '$lib/constants/strings';

	interface Props {
		hoursCompleted: number;
		hoursRequired: number;
	}

	let { hoursCompleted, hoursRequired }: Props = $props();

	const progressPercentage = $derived(
		hoursRequired > 0 ? Math.min(100, (hoursCompleted / hoursRequired) * 100) : 0
	);
</script>

<div class="progress-preview">
	<div class="progress-info">
		<span class="progress-label">{STRINGS.onboarding.yourProgress}</span>
		<span class="progress-numbers">{hoursCompleted} / {hoursRequired} hours</span>
	</div>
	<div class="progress-bar">
		<div
			class="progress-fill"
			style="width: {progressPercentage}%"
			role="progressbar"
			aria-valuenow={hoursCompleted}
			aria-valuemin="0"
			aria-valuemax={hoursRequired}
			aria-label="CE hours completion progress"
		></div>
	</div>
</div>

<style>
	.progress-preview {
		margin-top: 1rem;
		padding: 1rem;
		background: #f9fafb;
		border-radius: 8px;
	}

	.progress-info {
		display: flex;
		justify-content: space-between;
		align-items: center;
		margin-bottom: 0.5rem;
	}

	.progress-label {
		font-size: 0.875rem;
		font-weight: 500;
		color: #374151;
	}

	.progress-numbers {
		font-size: 0.875rem;
		font-weight: 600;
		color: #1f2937;
	}

	.progress-bar {
		width: 100%;
		height: 8px;
		background: #e5e7eb;
		border-radius: 4px;
		overflow: hidden;
	}

	.progress-fill {
		height: 100%;
		background: linear-gradient(90deg, #3B82F6 0%, #A855F7 100%);
		transition: width 0.3s ease;
	}
</style>
