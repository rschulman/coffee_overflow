<script lang="ts">
	import Card from '@smui/card';

	interface Props {
		title: string;
		subtitle: string;
		icon: string;
		iconBgColor: string;
		children: any;
	}

	let { title, subtitle, icon, iconBgColor, children }: Props = $props();
</script>

<Card class="stat-card">
	<div class="stat-header">
		<div>
			<h3 class="stat-title">{title}</h3>
			<p class="stat-subtitle">{subtitle}</p>
		</div>
		<div class="stat-icon" style="background: {iconBgColor}">
			<span class="material-icons">{icon}</span>
		</div>
	</div>
	<div class="stat-body">
		{@render children()}
	</div>
</Card>

<style>
	:global(.stat-card) {
		padding: 1.5rem !important;
	}

	.stat-header {
		display: flex;
		justify-content: space-between;
		align-items: flex-start;
		margin-bottom: 1.5rem;
	}

	.stat-title {
		font-size: 1rem;
		font-weight: 600;
		color: #1f2937;
		margin: 0 0 0.25rem 0;
	}

	.stat-subtitle {
		font-size: 0.875rem;
		color: #6b7280;
		margin: 0;
	}

	.stat-icon {
		width: 48px;
		height: 48px;
		border-radius: 12px;
		display: flex;
		align-items: center;
		justify-content: center;
	}

	.stat-icon .material-icons {
		font-size: 1.5rem;
	}

	.stat-body {
		text-align: center;
	}
</style>
