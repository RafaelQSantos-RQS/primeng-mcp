---
title: "Angular AutoFocus Directive"
type: "component"
version: "latest"
source_url: "https://primeng.dev/llms/components/autofocus.md"
description: "AutoFocus manages focus on focusable element on load."
---

# Angular AutoFocus Directive

AutoFocus manages focus on focusable element on load.

## Basic

AutoFocus is applied to any focusable input element with the pAutoFocus directive.

```typescript
import { Component } from '@angular/core';
import { InputTextModule } from 'primeng/inputtext';
import { AutoFocusModule } from 'primeng/autofocus';

@Component({
    template: `
        <div class="flex justify-center">
            <input type="text" pInputText [pAutoFocus]="true" placeholder="Automatically focused" />
        </div>
    `,
    standalone: true,
    imports: [InputTextModule, AutoFocusModule]
})
export class AutoFocusBasicDemo {}
```

## Auto Focus

AutoFocus manages focus on focusable element on load.

### Props

| Name | Type | Default | Description |
|------|------|---------|-------------|
| autofocus | boolean | - | When present, it specifies that the component should automatically get focus on load. |
| dt | object \| undefined | undefined | Defines scoped design tokens of the component. |
| unstyled | boolean \| undefined | undefined | Indicates whether the component should be rendered without styles. |
| pt | any | undefined | Used to pass attributes to DOM elements inside the component. |
| ptOptions | PassThroughOptions \| undefined | undefined | Used to configure passthrough(pt) options of the component. |
