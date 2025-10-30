<script lang="ts">
	import { STRINGS } from '$lib/constants/strings';

	interface Props {
		hoursCompleted: number;
		hoursRequired: number;
	}

	let { hoursCompleted, hoursRequired }: Props = $props();

	const progressPercentage = $derived(
		hoursRequired > 0 ? Math.round((hoursCompleted / hoursRequired) * 100) : 0
	);
	const strokeDasharray = $derived(`${progressPercentage * 2.827}, 282.7`);
</script>

<div class="progress-main">
	<div class="progress-circle">
		<svg viewBox="0 0 100 100" aria-hidden="true">
			<defs>
				<linearGradient id="progress-gradient" x1="0%" y1="0%" x2="100%" y2="100%">
					<stop offset="0%" style="stop-color:#3B82F6;stop-opacity:1" />
					<stop offset="100%" style="stop-color:#A855F7;stop-opacity:1" />
				</linearGradient>
			</defs>
			<circle cx="50" cy="50" r="45" class="progress-bg" />
			<circle
				cx="50"
				cy="50"
				r="45"
				class="progress-ring"
				style="stroke-dasharray: {strokeDasharray}"
			/>
		</svg>
		<div class="progress-value">
			<span class="progress-number">{progressPercentage}%</span>
		</div>
	</div>
	<div class="progress-details">
		<div class="progress-stat">
			<span class="progress-stat-value">{hoursCompleted}</span>
			<span class="progress-stat-label">{STRINGS.dashboard.completed}</span>
		</div>
		<div class="progress-divider"></div>
		<div class="progress-stat">
			<span class="progress-stat-value">{hoursRequired}</span>
			<span class="progress-stat-label">{STRINGS.dashboard.required}</span>
		</div>
		<div class="progress-divider"></div>
		<div class="progress-stat">
			<span class="progress-stat-value">{hoursRequired - hoursCompleted}</span>
			<span class="progress-stat-label">{STRINGS.dashboard.remaining}</span>
		</div>
	</div>
</div>

<style>
	.progress-main {
		display: flex;
		align-items: center;
		gap: 2rem;
	}

	.progress-circle {
		position: relative;
		width: 120px;
		height: 120px;
		flex-shrink: 0;
	}

	.progress-circle svg {
		width: 100%;
		height: 100%;
		transform: rotate(-90deg);
	}

	.progress-bg {
		fill: none;
		stroke: #e5e7eb;
		stroke-width: 8;
	}

	.progress-ring {
		fill: none;
		stroke: url(#progress-gradient);
		stroke-width: 8;
		stroke-linecap: round;
		transition: stroke-dasharray 0.3s ease;
	}

	.progress-value {
		position: absolute;
		top: 50%;
		left: 50%;
		transform: translate(-50%, -50%);
		text-align: center;
	}

	.progress-number {
		font-size: 1.75rem;
		font-weight: 700;
		background: linear-gradient(135deg, #3B82F6 0%, #A855F7 100%);
		-webkit-background-clip: text;
		-webkit-text-fill-color: transparent;
		background-clip: text;
	}

	.progress-details {
		display: flex;
		gap: 1.5rem;
		flex: 1;
	}

	.progress-stat {
		display: flex;
		flex-direction: column;
		gap: 0.25rem;
	}

	.progress-stat-value {
		font-size: 1.5rem;
		font-weight: 700;
		color: #1f2937;
	}

	.progress-stat-label {
		font-size: 0.875rem;
		color: #6b7280;
	}

	.progress-divider {
		width: 1px;
		background: #e5e7eb;
	}

	@media (max-width: 1024px) {
		.progress-main {
			flex-direction: column;
			text-align: center;
		}

		.progress-details {
			justify-content: center;
		}
	}
</style>
