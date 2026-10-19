---
title: "Angular IconField Component"
type: "component"
version: "latest"
source_url: "https://primeng.dev/llms/components/iconfield.md"
description: "IconField wraps an input and an icon."
---

# Angular IconField Component

IconField wraps an input and an icon.

## Accessibility

Screen Reader IconField and InputIcon does not require any roles and attributes. Keyboard Support Components does not include any interactive elements.

## Basic

An input field with an embedded leading or trailing icon.

```typescript
import { Component } from '@angular/core';
import { IconFieldModule } from 'primeng/iconfield';
import { InputIconModule } from 'primeng/inputicon';
import { InputTextModule } from 'primeng/inputtext';
import { Search } from '@primeicons/angular/search';
import { Spinner } from '@primeicons/angular/spinner';

@Component({
    template: `
        <div class="flex flex-wrap justify-center gap-4">
            <p-iconfield>
                <p-inputicon>
                    <svg data-p-icon="search" />
                </p-inputicon>
                <input type="text" pInputText placeholder="Search" />
            </p-iconfield>
            <p-iconfield>
                <input type="text" pInputText variant="filled" />
                <p-inputicon>
                    <svg data-p-icon="spinner" class="animate-spin" />
                </p-inputicon>
            </p-iconfield>
        </div>
    `,
    standalone: true,
    imports: [IconFieldModule, InputIconModule, InputTextModule, Search, Spinner]
})
export class IconFieldBasicDemo {}
```

## Clickable

Icons inside InputIcon can be interactive with event handlers like click.

```typescript
import { Component } from '@angular/core';
import { FormsModule } from '@angular/forms';
import { IconFieldModule } from 'primeng/iconfield';
import { InputIconModule } from 'primeng/inputicon';
import { InputTextModule } from 'primeng/inputtext';
import { Search } from '@primeicons/angular/search';
import { Times } from '@primeicons/angular/times';

@Component({
    template: `
        <div class="flex flex-wrap justify-center gap-4">
            <p-iconfield>
                <p-inputicon>
                    <svg data-p-icon="search" />
                </p-inputicon>
                <input type="text" pInputText [(ngModel)]="value" placeholder="Search" />
                <p-inputicon>
                    <svg data-p-icon="times" class="cursor-pointer" (click)="value = ''" />
                </p-inputicon>
            </p-iconfield>
        </div>
    `,
    standalone: true,
    imports: [IconFieldModule, InputIconModule, InputTextModule, FormsModule, Search, Times]
})
export class IconFieldClickableDemo {
    value: string = 'PrimeNG';
}
```

## floatlabel-doc

FloatLabel visually integrates a label with its form element. Visit FloatLabel documentation for more information.

```typescript
import { Component } from '@angular/core';
import { FormsModule } from '@angular/forms';
import { FloatLabelModule } from 'primeng/floatlabel';
import { IconFieldModule } from 'primeng/iconfield';
import { InputIconModule } from 'primeng/inputicon';
import { InputTextModule } from 'primeng/inputtext';
import { Search } from '@primeicons/angular/search';

@Component({
    template: `
        <div class="flex flex-wrap justify-center items-end gap-4">
            <p-floatlabel>
                <p-iconfield>
                    <p-inputicon>
                        <svg data-p-icon="search" />
                    </p-inputicon>
                    <input pInputText id="over_label" [(ngModel)]="value1" autocomplete="off" />
                </p-iconfield>
                <label for="over_label">Over Label</label>
            </p-floatlabel>
            <p-floatlabel variant="in">
                <p-iconfield>
                    <p-inputicon>
                        <svg data-p-icon="search" />
                    </p-inputicon>
                    <input pInputText id="in_label" [(ngModel)]="value2" autocomplete="off" variant="filled" />
                </p-iconfield>
                <label for="in_label">In Label</label>
            </p-floatlabel>
            <p-floatlabel variant="on">
                <p-iconfield>
                    <p-inputicon>
                        <svg data-p-icon="search" />
                    </p-inputicon>
                    <input pInputText id="on_label" [(ngModel)]="value3" autocomplete="off" />
                </p-iconfield>
                <label for="on_label">On Label</label>
            </p-floatlabel>
        </div>
    `,
    standalone: true,
    imports: [FloatLabelModule, IconFieldModule, InputIconModule, InputTextModule, FormsModule, Search]
})
export class IconFieldFloatLabelDemo {
    value1: string | undefined;

    value2: string | undefined;

    value3: string | undefined;
}
```

## iftalabel-doc

IftaLabel is used to create infield top aligned labels. Visit IftaLabel documentation for more information.

```typescript
import { Component } from '@angular/core';
import { FormsModule } from '@angular/forms';
import { IconFieldModule } from 'primeng/iconfield';
import { IftaLabelModule } from 'primeng/iftalabel';
import { InputIconModule } from 'primeng/inputicon';
import { InputTextModule } from 'primeng/inputtext';
import { Envelope } from '@primeicons/angular/envelope';
import { Check } from '@primeicons/angular/check';

@Component({
    template: `
        <div class="flex justify-center">
            <p-iftalabel>
                <p-iconfield>
                    <p-inputicon>
                        <svg data-p-icon="envelope" />
                    </p-inputicon>
                    <input pInputText id="email" [(ngModel)]="value" autocomplete="off" variant="filled" />
                    <p-inputicon>
                        <svg data-p-icon="check" />
                    </p-inputicon>
                </p-iconfield>
                <label for="email">Email</label>
            </p-iftalabel>
        </div>
    `,
    standalone: true,
    imports: [IconFieldModule, IftaLabelModule, InputIconModule, InputTextModule, FormsModule, Envelope, Check]
})
export class IconFieldIftaLabelDemo {
    value: string | undefined;
}
```

## preview-doc

```typescript
import { Component } from '@angular/core';
import { IconFieldModule } from 'primeng/iconfield';
import { InputIconModule } from 'primeng/inputicon';
import { InputTextModule } from 'primeng/inputtext';
import { InputPasswordModule } from 'primeng/inputpassword';
import { Search } from '@primeicons/angular/search';
import { Spinner } from '@primeicons/angular/spinner';
import { CreditCard } from '@primeicons/angular/credit-card';
import { Times } from '@primeicons/angular/times';
import { Lock } from '@primeicons/angular/lock';
import { Eye } from '@primeicons/angular/eye';

@Component({
    template: `
        <div class="flex flex-col gap-4 max-w-3xs w-full mx-auto">
            <p-iconfield>
                <p-inputicon>
                    <svg data-p-icon="search" />
                </p-inputicon>
                <input type="text" pInputText fluid placeholder="Search for everything" />
                <p-inputicon>
                    <kbd class="text-xs font-medium leading-none!">⌘+K</kbd>
                </p-inputicon>
            </p-iconfield>
            <p-iconfield>
                <input type="text" pInputText fluid variant="filled" placeholder="Processing..." />
                <p-inputicon>
                    <svg data-p-icon="spinner" class="animate-spin" />
                </p-inputicon>
            </p-iconfield>
            <p-iconfield>
                <p-inputicon>
                    <svg data-p-icon="credit-card" />
                </p-inputicon>
                <input type="text" pInputText fluid value="4352" />
                <p-inputicon>
                    <svg data-p-icon="times" />
                </p-inputicon>
            </p-iconfield>
            <p-iconfield>
                <p-inputicon>
                    <img alt="US" src="https://primefaces.org/cdn/primevue/images/flag/flag_placeholder.png" class="flag flag-us" style="width: 20px; height: 13.5px" />
                </p-inputicon>
                <input type="text" pInputText fluid placeholder="+1 (___) ___-____" />
            </p-iconfield>
            <p-iconfield>
                <p-inputicon>
                    <svg data-p-icon="lock" />
                </p-inputicon>
                <input type="password" pInputPassword fluid variant="filled" value="12345678" placeholder="Enter your password" />
                <p-inputicon>
                    <svg data-p-icon="eye" />
                </p-inputicon>
            </p-iconfield>
        </div>
    `,
    standalone: true,
    imports: [IconFieldModule, InputIconModule, InputTextModule, InputPasswordModule, Search, Spinner, CreditCard, Times, Lock, Eye]
})
export class IconFieldPreviewDemo {}
```

## Sizes

IconField is compatible with the size setting of the input field.

```typescript
import { Component } from '@angular/core';
import { FormsModule } from '@angular/forms';
import { IconFieldModule } from 'primeng/iconfield';
import { InputIconModule } from 'primeng/inputicon';
import { InputTextModule } from 'primeng/inputtext';
import { Search } from '@primeicons/angular/search';
import { User } from '@primeicons/angular/user';
import { Lock } from '@primeicons/angular/lock';
import { Spinner } from '@primeicons/angular/spinner';

@Component({
    template: `
        <div class="flex flex-col items-center gap-4">
            <p-iconfield>
                <p-inputicon>
                    <svg data-p-icon="search" />
                </p-inputicon>
                <input pInputText [(ngModel)]="value1" placeholder="Small" pSize="small" />
            </p-iconfield>
            <p-iconfield>
                <input pInputText [(ngModel)]="value2" placeholder="Normal" />
                <p-inputicon>
                    <svg data-p-icon="user" />
                </p-inputicon>
            </p-iconfield>
            <p-iconfield>
                <p-inputicon>
                    <svg data-p-icon="lock" />
                </p-inputicon>
                <input pInputText [(ngModel)]="value3" placeholder="Large" pSize="large" />
                <p-inputicon>
                    <svg data-p-icon="spinner" class="animate-spin" />
                </p-inputicon>
            </p-iconfield>
        </div>
    `,
    standalone: true,
    imports: [IconFieldModule, InputIconModule, InputTextModule, FormsModule, Search, User, Lock, Spinner]
})
export class IconFieldSizesDemo {
    value1: any = null;

    value2: any = null;

    value3: any = null;
}
```

## Template

An eye icon is displayed by default when the image is hovered in preview mode. Use the indicator template for custom content.

```typescript
import { Component } from '@angular/core';
import { IconFieldModule } from 'primeng/iconfield';
import { InputIconModule } from 'primeng/inputicon';
import { InputTextModule } from 'primeng/inputtext';

@Component({
    template: `
        <div class="flex justify-center">
            <p-iconfield iconPosition="left">
                <p-inputicon>
                    <svg width="14" height="16" viewBox="0 0 35 40" fill="none" xmlns="http://www.w3.org/2000/svg">
                        <path d="M15.1934 0V0V0L0.0391235 5.38288L2.35052 25.3417L15.1934 32.427V32.427V32.427L28.0364 25.3417L30.3478 5.38288L15.1934 0Z" fill="var(--p-primary-color)" />
                        <mask id="mask0_1_52" style="mask-type:luminance" maskUnits="userSpaceOnUse" x="0" y="0" width="31" height="33">
                            <path d="M15.1934 0V0V0L0.0391235 5.38288L2.35052 25.3417L15.1934 32.427V32.427V32.427L28.0364 25.3417L30.3478 5.38288L15.1934 0Z" fill="var(--ground-background)" />
                        </mask>
                        <g mask="url(#mask0_1_52)">
                            <path fill-rule="evenodd" clip-rule="evenodd" d="M15.1935 0V3.5994V3.58318V20.0075V20.0075V32.427V32.427L28.0364 25.3417L30.3478 5.38288L15.1935 0Z" fill="var(--p-primary-color)" />
                        </g>
                        <path d="M19.6399 15.3776L18.1861 15.0547L19.3169 16.6695V21.6755L23.1938 18.4458V12.9554L21.4169 13.6013L19.6399 15.3776Z" fill="var(--ground-background)" />
                        <path d="M10.5936 15.3776L12.0474 15.0547L10.9166 16.6695V21.6755L7.03966 18.4458V12.9554L8.81661 13.6013L10.5936 15.3776Z" fill="var(--ground-background)" />
                        <path
                            fill-rule="evenodd"
                            clip-rule="evenodd"
                            d="M11.3853 16.9726L12.6739 15.0309L13.4793 15.5163H16.7008L17.5061 15.0309L18.7947 16.9726V24.254L17.8283 25.7103L16.7008 26.843H13.4793L12.3518 25.7103L11.3853 24.254V16.9726Z"
                            fill="var(--ground-background)"
                        />
                        <path d="M19.3168 24.7437L21.4168 22.6444V20.5451L19.3168 22.3214V24.7437Z" fill="var(--ground-background)" />
                        <path d="M10.9166 24.7437L8.81662 22.6444V20.5451L10.9166 22.3214V24.7437Z" fill="var(--ground-background)" />
                        <path
                            fill-rule="evenodd"
                            clip-rule="evenodd"
                            d="M13.0167 5.68861L11.7244 8.7568L13.8244 14.8932H14.7936V5.68861H13.0167ZM15.4397 5.68861V14.8932H16.5706L18.5091 8.7568L17.2167 5.68861H15.4397Z"
                            fill="var(--ground-background)"
                        />
                        <path d="M13.8244 14.8932L6.87813 12.3094L5.90888 8.27235L11.8859 8.7568L13.9859 14.8932H13.8244Z" fill="var(--ground-background)" />
                        <path d="M16.5706 14.8932L23.5169 12.3094L24.4861 8.27235L18.3476 8.7568L16.4091 14.8932H16.5706Z" fill="var(--ground-background)" />
                        <path d="M18.8321 8.27235L22.2245 7.94938L19.9629 5.68861H17.7013L18.8321 8.27235Z" fill="var(--ground-background)" />
                        <path d="M11.4013 8.27235L8.00893 7.94938L10.2705 5.68861H12.5321L11.4013 8.27235Z" fill="var(--ground-background)" />
                    </svg>
                </p-inputicon>
                <input type="text" pInputText placeholder="Search" />
            </p-iconfield>
        </div>
    `,
    standalone: true,
    imports: [IconFieldModule, InputIconModule, InputTextModule]
})
export class IconFieldTemplateDemo {}
```

## Icon Field

IconField wraps an input and an icon.

### Props

| Name | Type | Default | Description |
|------|------|---------|-------------|
| dt | object \| undefined | undefined | Defines scoped design tokens of the component. |
| unstyled | boolean \| undefined | undefined | Indicates whether the component should be rendered without styles. |
| pt | PassThrough<I, IconFieldPassThroughOptions<I>> | undefined | Used to pass attributes to DOM elements inside the component. |
| ptOptions | PassThroughOptions \| undefined | undefined | Used to configure passthrough(pt) options of the component. |
| iconPosition | "right" \| "left" | - | Position of the icon. |

## Pass Through Options

| Name | Type | Description |
|------|------|-------------|
| host | PassThroughOption<HTMLElement, I> | Used to pass attributes to the host's DOM element. |
| root | PassThroughOption<HTMLSpanElement, I> | Used to pass attributes to the root's DOM element. |

## Theming

### CSS Classes

| Class | Description |
|-------|-------------|
| p-iconfield | Class name of the root element |

### Design Tokens

| Token | CSS Variable | Description |
|-------|--------------|-------------|
| iconfield.icon.color | --p-iconfield-icon-color | Color of icon |
