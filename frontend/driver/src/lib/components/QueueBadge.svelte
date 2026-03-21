<script lang="ts">
	let { waiting = 0, estimatedMinutes = 0 }: {
		waiting: number;
		estimatedMinutes: number;
	} = $props();
</script>

<div class="flex items-center gap-2 text-xs">
	{#if waiting === 0}
		<span class="flex items-center gap-1 rounded-full bg-success/10 px-2.5 py-1 font-medium text-success">
			<span class="h-1.5 w-1.5 rounded-full bg-success"></span>
			Trong
		</span>
	{:else}
		<span class="rounded-full bg-warning/10 px-2.5 py-1 font-medium text-warning">
			{waiting} dang cho
		</span>
		<span class="text-muted-foreground">~{estimatedMinutes}p</span>
	{/if}
</div>
