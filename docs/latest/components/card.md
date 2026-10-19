---
title: "Angular Card Component"
type: "component"
version: "latest"
source_url: "https://primeng.dev/llms/components/card.md"
description: "Card is a flexible container component."
---

# Angular Card Component

Card is a flexible container component.

## Accessibility

Screen Reader A card can be utilized in many use cases as a result no role is enforced, in fact a role may not be necessary if the card is used for presentational purposes only. Any valid attribute is passed to the container element so if you require to use one of the landmark roles like region , you may use the role property. Keyboard Support Component does not include any interactive elements.

## Advanced

Use the header template to place an image, avatar or other content in the header.

```typescript
import { Component } from '@angular/core';
import { AvatarModule } from 'primeng/avatar';
import { CardModule } from 'primeng/card';
import { TagModule } from 'primeng/tag';
import { ButtonModule } from 'primeng/button';
import { StarFill } from '@primeicons/angular/star-fill';
import { MapMarker } from '@primeicons/angular/map-marker';
import { Phone } from '@primeicons/angular/phone';
import { Globe } from '@primeicons/angular/globe';

@Component({
    template: `
        <div class="flex justify-center">
            <p-card class="max-w-sm w-full overflow-hidden" [pt]="{ body: { class: 'pt-16!' } }">
                <ng-template #header>
                    <div class="relative">
                        <img
                            class="w-full max-h-42 object-cover"
                            alt="user header"
                            src="https://images.unsplash.com/photo-1513649718256-1a7162666bad?q=80&w=2070&auto=format&fit=crop&ixlib=rb-4.1.0&ixid=M3wxMjA3fDB8MHxwaG90by1wYWdlfHx8fGVufDB8fHx8fA%3D%3D"
                        />
                        <p-avatar shape="circle" class="w-24! h-24! border-3 border-surface-0 dark:border-surface-900 absolute! -bottom-12! left-4!">
                            <img src="https://images.unsplash.com/photo-1722495178488-c8056c4ec2c0?q=80&w=2081&auto=format&fit=crop&ixlib=rb-4.1.0&ixid=M3wxMjA3fDB8MHxwaG90by1wYWdlfHx8fGVufDB8fHx8fA%3D%3D" />
                        </p-avatar>
                    </div>
                </ng-template>
                <ng-template #title>
                    <span class="font-bold text-xl">Sakura Fresh Market</span>
                </ng-template>
                <ng-template #subtitle>
                    <div class="flex items-center gap-2">
                        <p-tag severity="info" value="Daily" />
                        <p-tag severity="info" value="Premium" />
                    </div>
                </ng-template>
                <div class="space-y-4">
                    <p>Sakura Fresh Market is your go-to store for fresh local produce, Japanese snacks, and daily essentials — all in one place!</p>
                    <div class="flex items-center gap-2">
                        <svg data-p-icon="star-fill" class="text-yellow-500"></svg>
                        <span><b>4.6</b> (200+ reviews)</span>
                    </div>
                    <div class="flex items-center gap-2">
                        <svg data-p-icon="map-marker"></svg>
                        <span>Tokyo, Shibuya-ku</span>
                    </div>
                </div>
                <ng-template #footer>
                    <div class="flex items-center gap-2 mt-4">
                        <button type="button" pButton severity="secondary" [outlined]="true" class="flex-1">
                            <svg data-p-icon="phone"></svg>
                            Call Us
                        </button>
                        <button type="button" pButton class="flex-1">
                            <svg data-p-icon="globe"></svg>
                            Visit Site
                        </button>
                    </div>
                </ng-template>
            </p-card>
        </div>
    `,
    standalone: true,
    imports: [AvatarModule, CardModule, TagModule, ButtonModule, StarFill, MapMarker, Phone, Globe]
})
export class CardAdvancedDemo {}
```

## Basic

A container with a header, body, and footer for structured content.

```typescript
import { Component } from '@angular/core';
import { CardModule } from 'primeng/card';

@Component({
    template: `
        <div class="flex justify-center">
            <p-card class="max-w-sm w-full" header="Starter Plan" subheader="For personal projects">
                <p class="m-0">Includes essential features, basic analytics and access to community support.</p>
                <ng-template #footer>
                    <span class="text-sm text-surface-500 dark:text-surface-400">Cancel anytime. No credit card required.</span>
                </ng-template>
            </p-card>
        </div>
    `,
    standalone: true,
    imports: [CardModule]
})
export class CardBasicDemo {}
```

## preview-doc

```typescript
import { Component } from '@angular/core';
import { CardModule } from 'primeng/card';
import { ButtonModule } from 'primeng/button';

@Component({
    template: `
        <div class="flex justify-center">
            <p-card class="max-w-sm w-full" header="Pro Subscription" subheader="Everything you need to scale your workflow">
                <p>Get unlimited access to all features, priority support, and advanced analytics to help your team grow faster.</p>
                <ng-template #footer>
                    <div class="flex flex-col gap-4">
                        <span class="text-lg font-medium">$29 / month</span>
                        <button type="button" pButton class="w-full">Upgrade</button>
                    </div>
                </ng-template>
            </p-card>
        </div>
    `,
    standalone: true,
    imports: [CardModule, ButtonModule]
})
export class CardPreviewDemo {}
```

## With Form

Card can be used as a form container with input fields and action buttons.

```typescript
import { Component } from '@angular/core';
import { CardModule } from 'primeng/card';
import { ButtonModule } from 'primeng/button';
import { InputTextModule } from 'primeng/inputtext';
import { LabelModule } from 'primeng/label';

@Component({
    template: `
        <div class="flex justify-center">
            <p-card class="max-w-sm w-full" header="Welcome back" subheader="Sign in with your email to continue.">
                <form class="space-y-6 mt-3">
                    <div class="flex flex-col gap-2">
                        <label pLabel for="email">Email</label>
                        <input pInputText id="email" type="email" />
                    </div>
                    <div class="flex flex-col gap-2">
                        <div class="flex items-center justify-between">
                            <label pLabel for="password" class="flex-1">Password</label>
                            <button type="button" pButton variant="link" class="p-0">Forgot password?</button>
                        </div>
                        <input pInputText id="password" type="password" />
                    </div>
                </form>
                <ng-template #footer>
                    <div class="flex flex-col gap-4">
                        <button type="button" pButton class="w-full">Login</button>
                        <button type="button" pButton severity="secondary" [outlined]="true" class="w-full">Login with Google</button>
                        <div class="mt-2 text-center text-surface-500 text-sm">
                            Don't have an account?
                            <button type="button" pButton variant="link" class="p-0">Sign up</button>
                        </div>
                    </div>
                </ng-template>
            </p-card>
        </div>
    `,
    standalone: true,
    imports: [CardModule, ButtonModule, InputTextModule, LabelModule]
})
export class CardWithFormDemo {}
```

## Card

Card is a flexible container component.

### Props

| Name | Type | Default | Description |
|------|------|---------|-------------|
| dt | object \| undefined | undefined | Defines scoped design tokens of the component. |
| unstyled | boolean \| undefined | undefined | Indicates whether the component should be rendered without styles. |
| pt | PassThrough<I, CardPassThroughOptions<I>> | undefined | Used to pass attributes to DOM elements inside the component. |
| ptOptions | PassThroughOptions \| undefined | undefined | Used to configure passthrough(pt) options of the component. |
| header | string \| undefined | - | Header of the card. |
| subheader | string \| undefined | - | Subheader of the card. |

### Templates

| Name | Type | Description |
|------|------|-------------|
| header | TemplateRef<void> \| undefined | Custom header template. |
| title | TemplateRef<void> \| undefined | Custom title template. |
| subtitle | TemplateRef<void> \| undefined | Custom subtitle template. |
| content | TemplateRef<void> \| undefined | Custom content template. |
| footer | TemplateRef<void> \| undefined | Custom footer template. |

## Pass Through Options

| Name | Type | Description |
|------|------|-------------|
| root | PassThroughOption<HTMLElement, I> | Used to pass attributes to the root's DOM element. |
| header | PassThroughOption<HTMLDivElement, I> | Used to pass attributes to the header's DOM element. |
| body | PassThroughOption<HTMLDivElement, I> | Used to pass attributes to the body's DOM element. |
| subtitle | PassThroughOption<HTMLDivElement, I> | Used to pass attributes to the subtitle's DOM element. |
| title | PassThroughOption<HTMLDivElement, I> | Used to pass attributes to the title's DOM element. |
| content | PassThroughOption<HTMLDivElement, I> | Used to pass attributes to the content's DOM element. |
| footer | PassThroughOption<HTMLDivElement, I> | Used to pass attributes to the footer's DOM element. |

## Theming

### CSS Classes

| Class | Description |
|-------|-------------|
| p-card | Class name of the root element |
| p-card-header | Class name of the header element |
| p-card-body | Class name of the body element |
| p-card-caption | Class name of the caption element |
| p-card-title | Class name of the title element |
| p-card-subtitle | Class name of the subtitle element |
| p-card-content | Class name of the content element |
| p-card-footer | Class name of the footer element |

### Design Tokens

| Token | CSS Variable | Description |
|-------|--------------|-------------|
| card.background | --p-card-background | Background of root |
| card.border.radius | --p-card-border-radius | Border radius of root |
| card.color | --p-card-color | Color of root |
| card.shadow | --p-card-shadow | Shadow of root |
| card.body.padding | --p-card-body-padding | Padding of body |
| card.body.gap | --p-card-body-gap | Gap of body |
| card.caption.gap | --p-card-caption-gap | Gap of caption |
| card.title.font.size | --p-card-title-font-size | Font size of title |
| card.title.font.weight | --p-card-title-font-weight | Font weight of title |
| card.subtitle.color | --p-card-subtitle-color | Color of subtitle |
| card.subtitle.font.size | --p-card-subtitle-font-size | Font size of subtitle |
| card.subtitle.font.weight | --p-card-subtitle-font-weight | Font weight of subtitle |
