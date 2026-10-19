---
title: "Angular Bind Directive"
type: "component"
version: "latest"
source_url: "https://primeng.dev/llms/components/bind.md"
description: "The Bind directive dynamically applies DOM element attributes using properties defined in an object."
---

# Angular Bind Directive

The Bind directive dynamically applies DOM element attributes using properties defined in an object.

## Examples

The pBind is a directive that accepts an object of HTML attributes.

```typescript
import { Component, signal } from '@angular/core';
import { TooltipModule } from 'primeng/tooltip';

@Component({
    template: `
        <div class="flex flex-wrap justify-center gap-4">
            <div [pBind]="{ 'aria-label': 'basic-box', class: 'bg-primary text-primary-contrast flex items-center justify-center p-3 rounded-lg w-28 font-medium text-sm' }">Static</div>
            <div [pBind]="boxBinding()" pTooltip="Click to change color">Dynamic</div>
        </div>
    `,
    standalone: true,
    imports: [TooltipModule]
})
export class BindExamplesDemo {
    isActive = signal<boolean>(false);

    boxBinding = computed(() => ({
        class: ['flex items-center justify-center p-3 rounded-lg select-none w-24 border border-primary font-medium text-sm', this.isActive() ? 'bg-primary text-primary-contrast' : 'bg-transparent text-primary-600 dark:text-primary-400'],
        style: { cursor: 'pointer', transition: 'all 0.3s' },
        'data-state': this.isActive() ? 'active' : 'inactive',
        onclick: () => {
            this.isActive.set(!this.isActive());
        }
    }));
}
```

## Bind

Bind directive provides dynamic attribute, property, and event listener binding functionality.

### Props

| Name | Type | Default | Description |
|------|------|---------|-------------|
| pBind | { [key: string]: any } \| undefined | - | Dynamic attributes, properties, and event listeners to be applied to the host element. |
