---
title: "Angular InputGroup Component"
type: "component"
version: "latest"
source_url: "https://primeng.dev/llms/components/inputgroup.md"
description: "Text, icon, buttons and other content can be grouped next to an input."
---

# Angular InputGroup Component

Text, icon, buttons and other content can be grouped next to an input.

## Accessibility

Screen Reader InputGroup and InputGroupAddon does not require any roles and attributes. Keyboard Support Component does not include any interactive elements.

## Basic

Combines an input with addons such as buttons or text labels.

```typescript
import { Component } from '@angular/core';
import { FormsModule } from '@angular/forms';
import { InputGroupModule } from 'primeng/inputgroup';
import { InputGroupAddonModule } from 'primeng/inputgroupaddon';
import { InputTextModule } from 'primeng/inputtext';
import { User } from '@primeicons/angular/user';

@Component({
    template: `
        <div class="space-y-4 max-w-xs mx-auto">
            <p-inputgroup>
                <p-inputgroup-addon>
                    <svg data-p-icon="user" />
                </p-inputgroup-addon>
                <input pInputText [(ngModel)]="text1" placeholder="Username" />
            </p-inputgroup>
            <p-inputgroup>
                <p-inputgroup-addon>https://</p-inputgroup-addon>
                <input pInputText [(ngModel)]="text2" placeholder="website" />
                <p-inputgroup-addon>.com</p-inputgroup-addon>
            </p-inputgroup>
        </div>
    `,
    standalone: true,
    imports: [InputGroupModule, InputGroupAddonModule, InputTextModule, FormsModule, User]
})
export class InputGroupBasicDemo {
    text1: string | undefined;

    text2: string | undefined;
}
```

## Button

Buttons can be placed at either side of an input element.

```typescript
import { Component } from '@angular/core';
import { InputGroupModule } from 'primeng/inputgroup';
import { InputGroupAddonModule } from 'primeng/inputgroupaddon';
import { ButtonModule } from 'primeng/button';
import { InputTextModule } from 'primeng/inputtext';
import { Search } from '@primeicons/angular/search';
import { Check } from '@primeicons/angular/check';
import { Times } from '@primeicons/angular/times';

@Component({
    template: `
        <div class="space-y-4 max-w-xs mx-auto">
            <p-inputgroup>
                <button type="button" pButton>Search</button>
                <input pInputText placeholder="Keyword" />
                <p-inputgroup-addon class="text-xs">8 results</p-inputgroup-addon>
            </p-inputgroup>
            <p-inputgroup>
                <input pInputText placeholder="Keyword" />
                <p-inputgroup-addon>
                    <button type="button" pButton severity="secondary" variant="text" iconOnly>
                        <svg data-p-icon="search" />
                    </button>
                </p-inputgroup-addon>
            </p-inputgroup>
            <p-inputgroup>
                <p-inputgroup-addon>
                    <button type="button" pButton severity="secondary" iconOnly>
                        <svg data-p-icon="check" />
                    </button>
                </p-inputgroup-addon>
                <input pInputText placeholder="Vote" />
                <p-inputgroup-addon>
                    <button type="button" pButton severity="secondary" iconOnly>
                        <svg data-p-icon="times" />
                    </button>
                </p-inputgroup-addon>
            </p-inputgroup>
        </div>
    `,
    standalone: true,
    imports: [InputGroupModule, InputGroupAddonModule, ButtonModule, InputTextModule, Search, Check, Times]
})
export class InputGroupButtonDemo {}
```

## Checkbox & Radio

Checkbox and RadioButton components can be combined with an input element under the same group.

```typescript
import { Component } from '@angular/core';
import { FormsModule } from '@angular/forms';
import { CheckboxModule } from 'primeng/checkbox';
import { InputGroupModule } from 'primeng/inputgroup';
import { RadioButtonModule } from 'primeng/radiobutton';
import { InputGroupAddonModule } from 'primeng/inputgroupaddon';
import { InputTextModule } from 'primeng/inputtext';

@Component({
    template: `
        <div class="space-y-4 max-w-xs mx-auto">
            <p-inputgroup>
                <input type="text" pInputText placeholder="Price" />
                <p-inputgroup-addon><p-radiobutton [(ngModel)]="radioValue1" name="rb1" value="rb1" /></p-inputgroup-addon>
            </p-inputgroup>
            <p-inputgroup>
                <p-inputgroup-addon><p-checkbox [(ngModel)]="checked1" [binary]="true" /></p-inputgroup-addon>
                <input type="text" pInputText placeholder="Username" />
            </p-inputgroup>
            <p-inputgroup>
                <p-inputgroup-addon><p-checkbox [(ngModel)]="checked2" [binary]="true" /></p-inputgroup-addon>
                <input type="text" pInputText placeholder="Website" />
                <p-inputgroup-addon><p-radiobutton name="rb2" value="rb2" [(ngModel)]="category" /></p-inputgroup-addon>
            </p-inputgroup>
        </div>
    `,
    standalone: true,
    imports: [CheckboxModule, InputGroupModule, RadioButtonModule, InputGroupAddonModule, InputTextModule, FormsModule]
})
export class InputGroupCheckboxDemo {
    radioValue1: boolean = false;

    checked1: boolean = false;

    checked2: boolean = false;

    category: string | undefined;
}
```

## Float Label

FloatLabel visually integrates a label with its form element. Visit FloatLabel documentation for more information.

```typescript
import { Component } from '@angular/core';
import { FormsModule } from '@angular/forms';
import { FloatLabelModule } from 'primeng/floatlabel';
import { InputGroupModule } from 'primeng/inputgroup';
import { InputGroupAddonModule } from 'primeng/inputgroupaddon';
import { InputTextModule } from 'primeng/inputtext';
import { User } from '@primeicons/angular/user';

@Component({
    template: `
        <div class="space-y-4 max-w-xs mx-auto">
            <p-inputgroup>
                <p-inputgroup-addon>
                    <svg data-p-icon="user" />
                </p-inputgroup-addon>
                <p-floatlabel>
                    <input pInputText id="over_label" [(ngModel)]="value1" />
                    <label for="over_label">Over Label</label>
                </p-floatlabel>
            </p-inputgroup>
            <p-inputgroup>
                <p-inputgroup-addon>$</p-inputgroup-addon>
                <p-floatlabel variant="in">
                    <input pInputText id="in_label" [(ngModel)]="value2" />
                    <label for="in_label">In Label</label>
                </p-floatlabel>
                <p-inputgroup-addon>.00</p-inputgroup-addon>
            </p-inputgroup>
            <p-inputgroup>
                <p-inputgroup-addon>www</p-inputgroup-addon>
                <p-floatlabel variant="on">
                    <input pInputText id="on_label" [(ngModel)]="value3" />
                    <label for="on_label">On Label</label>
                </p-floatlabel>
            </p-inputgroup>
        </div>
    `,
    standalone: true,
    imports: [FloatLabelModule, InputGroupModule, InputGroupAddonModule, InputTextModule, FormsModule, User]
})
export class InputGroupFloatLabelDemo {
    value1: string | undefined;

    value2: string | undefined;

    value3: string | undefined;
}
```

## Ifta Label

IftaLabel is used to create infield top aligned labels. Visit IftaLabel documentation for more information.

```typescript
import { Component } from '@angular/core';
import { FormsModule } from '@angular/forms';
import { IftaLabelModule } from 'primeng/iftalabel';
import { InputGroupModule } from 'primeng/inputgroup';
import { InputGroupAddonModule } from 'primeng/inputgroupaddon';
import { InputTextModule } from 'primeng/inputtext';
import { User } from '@primeicons/angular/user';

@Component({
    template: `
        <div class="max-w-xs mx-auto">
            <p-inputgroup>
                <p-inputgroup-addon>
                    <svg data-p-icon="user" />
                </p-inputgroup-addon>
                <p-iftalabel>
                    <input pInputText id="name" [(ngModel)]="value" />
                    <label for="name">Name</label>
                </p-iftalabel>
            </p-inputgroup>
        </div>
    `,
    standalone: true,
    imports: [IftaLabelModule, InputGroupModule, InputGroupAddonModule, InputTextModule, FormsModule, User]
})
export class InputGroupIftaLabelDemo {
    value: string = 'Amy';
}
```

## Multiple

A group is created by wrapping the input and add-ons with the p-inputgroup component. Each add-on element is defined as a child of p-inputgroup-addon component. Multiple add-ons can be placed inside the same group.

```typescript
import { Component } from '@angular/core';
import { FormsModule } from '@angular/forms';
import { InputGroupModule } from 'primeng/inputgroup';
import { InputNumberModule } from 'primeng/inputnumber';
import { InputGroupAddonModule } from 'primeng/inputgroupaddon';
import { Clock } from '@primeicons/angular/clock';
import { StarFill } from '@primeicons/angular/star-fill';

@Component({
    template: `
        <div class="max-w-sm mx-auto">
            <p-inputgroup>
                <p-inputgroup-addon>
                    <svg data-p-icon="clock" />
                </p-inputgroup-addon>
                <p-inputgroup-addon>
                    <svg data-p-icon="star-fill" />
                </p-inputgroup-addon>
                <p-inputnumber [(ngModel)]="value" placeholder="Price" />
                <p-inputgroup-addon>$</p-inputgroup-addon>
                <p-inputgroup-addon>.00</p-inputgroup-addon>
            </p-inputgroup>
        </div>
    `,
    standalone: true,
    imports: [InputGroupModule, InputNumberModule, InputGroupAddonModule, FormsModule, Clock, StarFill]
})
export class InputGroupMultipleDemo {
    value: number | undefined;
}
```

## preview-doc

```typescript
import { Component } from '@angular/core';
import { FormsModule } from '@angular/forms';
import { SelectModule } from 'primeng/select';
import { InputGroupModule } from 'primeng/inputgroup';
import { InputGroupAddonModule } from 'primeng/inputgroupaddon';
import { InputTextModule } from 'primeng/inputtext';
import { User } from '@primeicons/angular/user';

interface PhoneCode {
    name: string;
    code: string;
    dial: string;
}

@Component({
    template: `
        <div class="space-y-4 max-w-xs mx-auto">
            <p-inputgroup>
                <p-inputgroup-addon>
                    <svg data-p-icon="user" />
                </p-inputgroup-addon>
                <input pInputText placeholder="Username" />
            </p-inputgroup>
            <p-inputgroup>
                <p-inputgroup-addon>https://</p-inputgroup-addon>
                <input pInputText placeholder="website" />
                <p-inputgroup-addon>.com</p-inputgroup-addon>
            </p-inputgroup>
            <p-inputgroup>
                <p-inputgroup-addon class="p-0!">
                    <p-select [(ngModel)]="selectedCode" [options]="phoneCodes" optionLabel="dial" scrollHeight="auto" class="border-none! shadow-none! bg-transparent! w-24" size="small">
                        <ng-template #selectedItem let-item>
                            <div class="flex items-center gap-1.5">
                                <img [alt]="item.name" src="https://primefaces.org/cdn/primevue/images/flag/flag_placeholder.png" [class]="'flag flag-' + item.code" style="width: 18px; height: 12px" />
                                <span class="text-sm">{{ item.dial }}</span>
                            </div>
                        </ng-template>
                        <ng-template #item let-item>
                            <div class="flex items-center gap-2">
                                <img [alt]="item.name" src="https://primefaces.org/cdn/primevue/images/flag/flag_placeholder.png" [class]="'flag flag-' + item.code" style="width: 18px; height: 12px" />
                                <span class="ml-auto">{{ item.dial }}</span>
                            </div>
                        </ng-template>
                    </p-select>
                </p-inputgroup-addon>
                <input pInputText placeholder="Phone number" />
            </p-inputgroup>
        </div>
    `,
    standalone: true,
    imports: [SelectModule, InputGroupModule, InputGroupAddonModule, InputTextModule, FormsModule, User]
})
export class InputGroupPreviewDemo {
    phoneCodes: PhoneCode[] = [
        { name: 'US', code: 'us', dial: '+1' },
        { name: 'GB', code: 'gb', dial: '+44' },
        { name: 'DE', code: 'de', dial: '+49' },
        { name: 'FR', code: 'fr', dial: '+33' },
        { name: 'TR', code: 'tr', dial: '+90' },
        { name: 'JP', code: 'jp', dial: '+81' },
        { name: 'BR', code: 'br', dial: '+55' },
        { name: 'IN', code: 'in', dial: '+91' }
    ];

    selectedCode: PhoneCode = this.phoneCodes[4];
}
```

## Select

A Select component can be used within an InputGroup alongside other add-ons and inputs.

```typescript
import { Component } from '@angular/core';
import { FormsModule } from '@angular/forms';
import { SelectModule } from 'primeng/select';
import { InputGroupModule } from 'primeng/inputgroup';
import { InputGroupAddonModule } from 'primeng/inputgroupaddon';
import { InputTextModule } from 'primeng/inputtext';
import { MapMarker } from '@primeicons/angular/map-marker';

interface City {
    label: string;
    value: string;
}

@Component({
    template: `
        <div class="space-y-4 max-w-md mx-auto">
            <p-inputgroup>
                <p-inputgroup-addon>
                    <svg data-p-icon="map-marker" />
                </p-inputgroup-addon>
                <p-select [(ngModel)]="city1" [options]="cities" optionLabel="label" optionValue="value" placeholder="Select a City" class="flex-1" />
            </p-inputgroup>
            <p-inputgroup>
                <p-inputgroup-addon>www</p-inputgroup-addon>
                <input pInputText [(ngModel)]="website" placeholder="Website" class="border-r-0!" />
                <p-select [(ngModel)]="city2" [options]="cities" optionLabel="label" optionValue="value" placeholder="Select a City" class="flex-1" />
            </p-inputgroup>
        </div>
    `,
    standalone: true,
    imports: [SelectModule, InputGroupModule, InputGroupAddonModule, InputTextModule, FormsModule, MapMarker]
})
export class InputGroupSelectDemo {
    city1: string | undefined;

    city2: string | undefined;

    website: string | undefined;

    cities: City[] = [
        { label: 'New York', value: 'ny' },
        { label: 'Rome', value: 'rm' },
        { label: 'London', value: 'ldn' },
        { label: 'Istanbul', value: 'ist' },
        { label: 'Paris', value: 'prs' }
    ];
}
```

## Input Group

InputGroup displays text, icon, buttons and other content can be grouped next to an input.

### Props

| Name | Type | Default | Description |
|------|------|---------|-------------|
| dt | object \| undefined | undefined | Defines scoped design tokens of the component. |
| unstyled | boolean \| undefined | undefined | Indicates whether the component should be rendered without styles. |
| pt | PassThrough<I, InputGroupPassThroughOptions<I>> | undefined | Used to pass attributes to DOM elements inside the component. |
| ptOptions | PassThroughOptions \| undefined | undefined | Used to configure passthrough(pt) options of the component. |

## Pass Through Options

| Name | Type | Description |
|------|------|-------------|
| host | PassThroughOption<HTMLElement, I> | Used to pass attributes to the host's DOM element. |
| root | PassThroughOption<HTMLElement, I> | Used to pass attributes to the root's DOM element. |

## Theming

### CSS Classes

| Class | Description |
|-------|-------------|
| p-inputgroup | Class name of the root element |

### Design Tokens

| Token | CSS Variable | Description |
|-------|--------------|-------------|
| inputgroup.addon.background | --p-inputgroup-addon-background | Background of addon |
| inputgroup.addon.border.color | --p-inputgroup-addon-border-color | Border color of addon |
| inputgroup.addon.color | --p-inputgroup-addon-color | Color of addon |
| inputgroup.addon.border.radius | --p-inputgroup-addon-border-radius | Border radius of addon |
| inputgroup.addon.padding | --p-inputgroup-addon-padding | Padding of addon |
| inputgroup.addon.min.width | --p-inputgroup-addon-min-width | Min width of addon |
| inputgroup.addon.font.weight | --p-inputgroup-addon-font-weight | Font weight of addon |
| inputgroup.addon.font.size | --p-inputgroup-addon-font-size | Font size of addon |
