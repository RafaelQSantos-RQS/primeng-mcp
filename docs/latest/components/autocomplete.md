---
title: "Angular AutoComplete Component"
type: "component"
version: "latest"
source_url: "https://primeng.dev/llms/components/autocomplete.md"
description: "AutoComplete is an input component that provides real-time suggestions when being typed."
---

# Angular AutoComplete Component

AutoComplete is an input component that provides real-time suggestions when being typed.

## Accessibility

Screen Reader Value to describe the component can either be provided via label tag combined with inputId prop or using ariaLabelledBy , ariaLabel props. The input element has combobox role in addition to aria-autocomplete , aria-haspopup and aria-expanded attributes. The relation between the input and the popup is created with aria-controls and aria-activedescendant attribute is used to instruct screen reader which option to read during keyboard navigation within the popup list. In multiple mode, chip list uses listbox role whereas each chip has the option role with aria-label set to the label of the chip. The popup list has an id that refers to the aria-controls attribute of the input element and uses listbox as the role. Each list item has option role and an id to match the aria-activedescendant of the input element.

## Basic

AutoComplete uses ngModel for two-way binding, requires a list of suggestions and a completeMethod to query for the results. The completeMethod gets the query text as event.query property and should update the suggestions with the search results. Use optionLabel to display a property of an object as the suggestion label.

```typescript
import { Component } from '@angular/core';
import { FormsModule } from '@angular/forms';
import { AutoCompleteModule } from 'primeng/autocomplete';

interface Command {
    label: string;
    shortcut: string;
}

@Component({
    template: `
        <div class="flex justify-center">
            <p-autocomplete [(ngModel)]="selectedCommand" [suggestions]="filteredCommands" (completeMethod)="search($event)" optionLabel="label" placeholder="Type a command..." inputStyleClass="w-full md:w-56" scrollHeight="14rem" />
        </div>
    `,
    standalone: true,
    imports: [AutoCompleteModule, FormsModule]
})
export class AutoCompleteBasicDemo {
    selectedCommand: Command | undefined;

    filteredCommands: Command[] = [];

    commands: Command[] = [
        { label: 'New File', shortcut: '⌘N' },
        { label: 'Open File', shortcut: '⌘O' },
        { label: 'Save', shortcut: '⌘S' },
        { label: 'Save As', shortcut: '⇧⌘S' },
        { label: 'Find', shortcut: '⌘F' },
        { label: 'Replace', shortcut: '⌘H' },
        { label: 'Go to Line', shortcut: '⌘G' },
        { label: 'Toggle Sidebar', shortcut: '⌘B' },
        { label: 'Split Editor', shortcut: '⌘\\' },
        { label: 'Close Tab', shortcut: '⌘W' }
    ];

    search(event: AutoCompleteCompleteEvent) {
        const query = event.query.toLowerCase();

        this.filteredCommands = query ? this.commands.filter((cmd) => cmd.label.toLowerCase().includes(query)) : [...this.commands];
    }
}
```

## clear-icon-doc

When showClear is enabled, a clear icon is displayed to reset the value.

```typescript
import { Component } from '@angular/core';
import { FormsModule } from '@angular/forms';
import { AutoCompleteModule } from 'primeng/autocomplete';
import { TagModule } from 'primeng/tag';

interface ProductCategory {
    label: string;
    value: string;
    count: number;
}

@Component({
    template: `
        <div class="flex justify-center">
            <p-autocomplete
                [(ngModel)]="selectedCategory"
                [suggestions]="filteredCategories"
                (completeMethod)="search($event)"
                optionLabel="label"
                [showClear]="true"
                placeholder="Search categories..."
                inputStyleClass="w-full md:w-56"
                scrollHeight="14rem"
            >
                <ng-template let-category #item>
                    <div class="flex items-center justify-between w-full">
                        <span>{{ category.label }}</span>
                        <p-tag [value]="category.count.toString()" severity="secondary" [rounded]="true" />
                    </div>
                </ng-template>
            </p-autocomplete>
        </div>
    `,
    standalone: true,
    imports: [AutoCompleteModule, TagModule, FormsModule]
})
export class AutoCompleteClearIconDemo {
    selectedCategory: ProductCategory | undefined;

    filteredCategories: ProductCategory[] = [];

    productCategories: ProductCategory[] = [
        { label: 'Electronics', value: 'electronics', count: 1247 },
        { label: 'Clothing', value: 'clothing', count: 856 },
        { label: 'Garden', value: 'home', count: 634 },
        { label: 'Sports', value: 'sports', count: 421 },
        { label: 'Books', value: 'books', count: 2103 },
        { label: 'Toys', value: 'toys', count: 312 }
    ];

    search(event: AutoCompleteCompleteEvent) {
        const query = event.query.toLowerCase();

        this.filteredCategories = query ? this.productCategories.filter((cat) => cat.label.toLowerCase().includes(query)) : [...this.productCategories];
    }
}
```

## custom-group-doc

Customize group headers with the group template.

```typescript
import { Component } from '@angular/core';
import { FormsModule } from '@angular/forms';
import { AutoCompleteModule } from 'primeng/autocomplete';
import { LabelModule } from 'primeng/label';
import { SelectItemGroup } from 'primeng/api';

@Component({
    template: `
        <div class="flex justify-center">
            <p-autocomplete
                [(ngModel)]="selectedCity"
                [group]="true"
                [suggestions]="filteredGroups"
                (completeMethod)="search($event)"
                optionLabel="label"
                optionGroupLabel="label"
                optionGroupChildren="items"
                placeholder="Search a city..."
                inputStyleClass="w-full md:w-56"
                scrollHeight="18rem"
                [pt]="{ optionGroup: 'border-b border-surface' }"
            >
                <ng-template let-group #group>
                    <div class="flex items-center gap-2">
                        <span>{{ countryFlags[group.value] }}</span>
                        <span>{{ group.label }}</span>
                    </div>
                </ng-template>
            </p-autocomplete>
        </div>
    `,
    standalone: true,
    imports: [AutoCompleteModule, LabelModule, FormsModule]
})
export class AutoCompleteCustomGroupDemo {
    selectedCity: any;

    filteredGroups: SelectItemGroup[] = [];

    countryFlags: Record<string, string> = {
        de: '🇩🇪',
        us: '🇺🇸',
        jp: '🇯🇵'
    };

    groupedCities: SelectItemGroup[] = [
        {
            label: 'Germany',
            value: 'de',
            items: [
                { label: 'Berlin', value: 'Berlin' },
                { label: 'Frankfurt', value: 'Frankfurt' },
                { label: 'Hamburg', value: 'Hamburg' }
            ]
        },
        {
            label: 'USA',
            value: 'us',
            items: [
                { label: 'Chicago', value: 'Chicago' },
                { label: 'Los Angeles', value: 'Los Angeles' },
                { label: 'New York', value: 'New York' }
            ]
        },
        {
            label: 'Japan',
            value: 'jp',
            items: [
                { label: 'Kyoto', value: 'Kyoto' },
                { label: 'Osaka', value: 'Osaka' },
                { label: 'Tokyo', value: 'Tokyo' }
            ]
        }
    ];

    search(event: AutoCompleteCompleteEvent) {
        const query = event.query.toLowerCase();
        const filtered: SelectItemGroup[] = [];

        for (const group of this.groupedCities) {
            const filteredItems = query ? group.items.filter((item) => item.label.toLowerCase().includes(query)) : group.items;

            if (filteredItems.length) {
                filtered.push({ ...group, items: filteredItems });
            }
        }

        this.filteredGroups = filtered;
    }
}
```

## custom-option-doc

Customize option content with the item template.

```typescript
import { Component } from '@angular/core';
import { FormsModule } from '@angular/forms';
import { AutoCompleteModule } from 'primeng/autocomplete';
import { AvatarModule } from 'primeng/avatar';
import { Product } from '@/domain/product';

interface TeamMember {
    id: number;
    name: string;
    role: string;
    avatar: string;
    status: 'online' | 'away' | 'offline';
    statusClass: string;
}

@Component({
    template: `
        <div class="flex justify-center">
            <p-autocomplete [(ngModel)]="selectedMember" [suggestions]="filteredMembers" (completeMethod)="search($event)" optionLabel="name" placeholder="Search team members..." inputStyleClass="w-full md:w-56" scrollHeight="16rem">
                <ng-template let-member #item>
                    <div class="flex items-center gap-3 py-1">
                        <div class="relative">
                            <p-avatar [label]="member.avatar" shape="circle" class="w-8 h-8" />
                            <span class="absolute -bottom-0.5 -right-0.5 w-3 h-3 rounded-full border-2 border-surface-0 dark:border-surface-900 {{ member.statusClass }}"></span>
                        </div>
                        <div class="flex flex-col">
                            <span class="font-medium">{{ member.name }}</span>
                            <span class="text-xs text-surface-500 dark:text-surface-400">{{ member.role }}</span>
                        </div>
                    </div>
                </ng-template>
            </p-autocomplete>
        </div>
    `,
    standalone: true,
    imports: [AutoCompleteModule, AvatarModule, FormsModule]
})
export class AutoCompleteCustomOptionDemo {
    selectedMember: TeamMember | undefined;

    filteredMembers: TeamMember[] = [];

    teamMembers: TeamMember[] = [
        { id: 1, name: 'Sarah Chen', role: 'Engineering Lead', avatar: 'SC', status: 'online', statusClass: 'bg-green-400' },
        { id: 2, name: 'Alex Rivera', role: 'Senior Developer', avatar: 'AR', status: 'online', statusClass: 'bg-green-400' },
        { id: 3, name: 'Jordan Kim', role: 'UX Designer', avatar: 'JK', status: 'away', statusClass: 'bg-amber-400' },
        { id: 4, name: 'Taylor Morgan', role: 'Product Manager', avatar: 'TM', status: 'offline', statusClass: 'bg-zinc-400' },
        { id: 5, name: 'Morgan Lee', role: 'DevOps Engineer', avatar: 'ML', status: 'online', statusClass: 'bg-green-400' },
        { id: 6, name: 'Casey Jones', role: 'QA Engineer', avatar: 'CJ', status: 'away', statusClass: 'bg-amber-400' }
    ];

    search(event: AutoCompleteCompleteEvent) {
        const query = event.query.toLowerCase();

        this.filteredMembers = query ? this.teamMembers.filter((m) => m.name.toLowerCase().includes(query) || m.role.toLowerCase().includes(query)) : [...this.teamMembers];
    }
}
```

## Disabled

When disabled is present, the element cannot be edited and focused.

```typescript
import { Component } from '@angular/core';
import { FormsModule } from '@angular/forms';
import { AutoCompleteModule } from 'primeng/autocomplete';

@Component({
    template: `
        <div class="flex justify-center">
            <p-autocomplete [(ngModel)]="selectedItem" [suggestions]="suggestions" placeholder="Disabled" (completeMethod)="search($event)" [disabled]="true" inputStyleClass="w-full md:w-56" scrollHeight="14rem" />
        </div>
    `,
    standalone: true,
    imports: [AutoCompleteModule, FormsModule]
})
export class AutoCompleteDisabledDemo {
    items: any[] | undefined;

    selectedItem: any;

    suggestions: any[] | undefined;

    search(event: AutoCompleteCompleteEvent) {
        this.suggestions = [...Array(10).keys()].map((item) => event.query + '-' + item);
    }
}
```

## Dropdown

Enabling dropdown property displays a trigger button next to the input field. The dropdownMode property defines the behavior of the button, blank sends an empty query and current sends the current value of the input.

```typescript
import { Component } from '@angular/core';
import { FormsModule } from '@angular/forms';
import { AutoCompleteModule } from 'primeng/autocomplete';

@Component({
    template: `
        <div class="flex justify-center">
            <p-autocomplete [(ngModel)]="value" [dropdown]="true" [suggestions]="items" (completeMethod)="search($event)" inputStyleClass="w-full md:w-56" scrollHeight="14rem" />
        </div>
    `,
    standalone: true,
    imports: [AutoCompleteModule, FormsModule]
})
export class AutoCompleteDropdownDemo {
    items: string[] | undefined;

    value: any;

    search(event: AutoCompleteCompleteEvent) {
        this.items = event.query ? [...Array(10).keys()].map((item) => event.query + '-' + item) : [...Array(10).keys()].map(String);
    }
}
```

## Filled

Specify the variant property as filled to display the component with a higher visual emphasis than the default outlined style.

```typescript
import { Component } from '@angular/core';
import { FormsModule } from '@angular/forms';
import { AutoCompleteModule } from 'primeng/autocomplete';

@Component({
    template: `
        <div class="flex justify-center">
            <p-autocomplete [(ngModel)]="selectedItem" [suggestions]="suggestions" (completeMethod)="search($event)" variant="filled" inputStyleClass="w-full md:w-56" scrollHeight="14rem" />
        </div>
    `,
    standalone: true,
    imports: [AutoCompleteModule, FormsModule]
})
export class AutoCompleteFilledDemo {
    items: any[] | undefined;

    selectedItem: any;

    suggestions: any[] | undefined;

    search(event: AutoCompleteCompleteEvent) {
        this.suggestions = [...Array(10).keys()].map((item) => event.query + '-' + item);
    }
}
```

## float-label-doc

A floating label appears on top of the input field when focused. Visit FloatLabel documentation for more information.

```typescript
import { Component } from '@angular/core';
import { FormsModule } from '@angular/forms';
import { AutoCompleteModule } from 'primeng/autocomplete';
import { FloatLabelModule } from 'primeng/floatlabel';

@Component({
    template: `
        <div class="flex flex-wrap justify-center items-end gap-4">
            <p-floatlabel>
                <p-autocomplete [(ngModel)]="value1" [suggestions]="items" (completeMethod)="search($event)" inputId="over_label" scrollHeight="14rem" />
                <label for="over_label">Over Label</label>
            </p-floatlabel>
            <p-floatlabel variant="in">
                <p-autocomplete [(ngModel)]="value2" [suggestions]="items" (completeMethod)="search($event)" inputId="in_label" scrollHeight="14rem" />
                <label for="in_label">In Label</label>
            </p-floatlabel>
            <p-floatlabel variant="on">
                <p-autocomplete [(ngModel)]="value3" [suggestions]="items" (completeMethod)="search($event)" inputId="on_label" scrollHeight="14rem" />
                <label for="on_label">On Label</label>
            </p-floatlabel>
        </div>
    `,
    standalone: true,
    imports: [AutoCompleteModule, FloatLabelModule, FormsModule]
})
export class AutoCompleteFloatLabelDemo {
    value1: string | undefined;

    value2: string | undefined;

    value3: string | undefined;

    items: any[] | undefined;

    search(event: AutoCompleteCompleteEvent) {
        this.items = [...Array(10).keys()].map((item) => event.query + '-' + item);
    }
}
```

## Fluid

The fluid prop makes the component take up the full width of its container when set to true.

```typescript
import { Component } from '@angular/core';
import { FormsModule } from '@angular/forms';
import { AutoCompleteModule } from 'primeng/autocomplete';

@Component({
    template: `
        <div>
            <p-autocomplete [(ngModel)]="value" [suggestions]="items" (completeMethod)="search($event)" fluid scrollHeight="14rem" />
        </div>
    `,
    standalone: true,
    imports: [AutoCompleteModule, FormsModule]
})
export class AutoCompleteFluidDemo {
    items: any[] = [];

    value: any;

    search(event: AutoCompleteCompleteEvent) {
        this.items = [...Array(10).keys()].map((item) => event.query + '-' + item);
    }
}
```

## focus-behavior-doc

Customize keyboard and mouse focus behavior with autoOptionFocus , selectOnFocus , and focusOnHover properties.

```typescript
import { Component } from '@angular/core';
import { FormsModule } from '@angular/forms';
import { AutoCompleteModule } from 'primeng/autocomplete';
import { ButtonModule } from 'primeng/button';

@Component({
    template: `
        <div class="space-y-4">
            <div class="flex flex-wrap gap-2 justify-center">
                <button pButton size="small" [severity]="autoOptionFocus ? 'primary' : 'secondary'" (click)="autoOptionFocus = !autoOptionFocus">autoOptionFocus: {{ autoOptionFocus }}</button>
                <button pButton size="small" [severity]="selectOnFocus ? 'primary' : 'secondary'" (click)="selectOnFocus = !selectOnFocus">selectOnFocus: {{ selectOnFocus }}</button>
                <button pButton size="small" [severity]="focusOnHover ? 'primary' : 'secondary'" (click)="focusOnHover = !focusOnHover">focusOnHover: {{ focusOnHover }}</button>
            </div>
            <div class="flex justify-center">
                <p-autocomplete
                    [(ngModel)]="value"
                    [suggestions]="items"
                    (completeMethod)="search($event)"
                    [autoOptionFocus]="autoOptionFocus"
                    [selectOnFocus]="selectOnFocus"
                    [focusOnHover]="focusOnHover"
                    placeholder="Search a city..."
                    inputStyleClass="w-full md:w-56"
                    scrollHeight="14rem"
                />
            </div>
        </div>
    `,
    standalone: true,
    imports: [AutoCompleteModule, ButtonModule, FormsModule]
})
export class AutoCompleteFocusBehaviorDemo {
    items: string[] = [];

    value: any;

    autoOptionFocus: boolean = true;

    selectOnFocus: boolean = false;

    focusOnHover: boolean = true;

    search(event: AutoCompleteCompleteEvent) {
        this.items = [...Array(10).keys()].map((item) => event.query + '-' + item);
    }
}
```

## force-selection-doc

ForceSelection mode validates the manual input to check whether it also exists in the suggestions list, if not the input value is cleared to make sure the value passed to the model is always one of the suggestions.

```typescript
import { Component, OnInit, inject } from '@angular/core';
import { FormsModule } from '@angular/forms';
import { AutoCompleteModule } from 'primeng/autocomplete';
import { CountryService } from '@/service/countryservice';

@Component({
    template: `
        <div class="flex justify-center">
            <p-autocomplete [(ngModel)]="selectedCountry" [forceSelection]="true" [suggestions]="filteredCountries" (completeMethod)="filterCountry($event)" optionLabel="name" inputStyleClass="w-full md:w-56" scrollHeight="14rem" />
        </div>
    `,
    standalone: true,
    imports: [AutoCompleteModule, FormsModule],
    providers: [CountryService]
})
export class AutoCompleteForceSelectionDemo implements OnInit {
    private countryService = inject(CountryService);

    countries: any[] | undefined;

    selectedCountry: any;

    filteredCountries: any[] | undefined;

    ngOnInit() {
        this.countryService.getCountries().then((countries) => {
            this.countries = countries;
        });
    }

    filterCountry(event: AutoCompleteCompleteEvent) {
        let filtered: any[] = [];
        let query = event.query;

        for (let i = 0; i < (this.countries as any[]).length; i++) {
            let country = (this.countries as any[])[i];

            if (country.name.toLowerCase().indexOf(query.toLowerCase()) == 0) {
                filtered.push(country);
            }
        }

        this.filteredCountries = filtered;
    }
}
```

## Group

Options can be grouped by category using the group property with optionGroupLabel and optionGroupChildren .

```typescript
import { Component } from '@angular/core';
import { FormsModule } from '@angular/forms';
import { AutoCompleteModule } from 'primeng/autocomplete';
import { LabelModule } from 'primeng/label';

interface Technology {
    label: string;
    value: string;
}

interface TechCategory {
    label: string;
    code: string;
    items: Technology[];
}

@Component({
    template: `
        <div class="flex justify-center">
            <p-autocomplete
                [(ngModel)]="selectedTech"
                [group]="true"
                [suggestions]="filteredGroups"
                (completeMethod)="search($event)"
                optionLabel="label"
                optionGroupLabel="label"
                optionGroupChildren="items"
                placeholder="Search technologies..."
                inputStyleClass="w-full md:w-56"
                scrollHeight="14rem"
            />
        </div>
    `,
    standalone: true,
    imports: [AutoCompleteModule, LabelModule, FormsModule]
})
export class AutoCompleteGroupDemo {
    selectedTech: any;

    filteredGroups: TechCategory[] = [];

    techStack: TechCategory[] = [
        {
            label: 'Frontend',
            code: 'FE',
            items: [
                { label: 'React', value: 'react' },
                { label: 'Vue', value: 'vue' },
                { label: 'Angular', value: 'angular' },
                { label: 'Svelte', value: 'svelte' }
            ]
        },
        {
            label: 'Backend',
            code: 'BE',
            items: [
                { label: 'Node.js', value: 'nodejs' },
                { label: 'Python', value: 'python' },
                { label: 'Java', value: 'java' },
                { label: 'Go', value: 'go' }
            ]
        },
        {
            label: 'Database',
            code: 'DB',
            items: [
                { label: 'PostgreSQL', value: 'postgresql' },
                { label: 'MongoDB', value: 'mongodb' },
                { label: 'Redis', value: 'redis' },
                { label: 'MySQL', value: 'mysql' }
            ]
        }
    ];

    search(event: AutoCompleteCompleteEvent) {
        const query = event.query.toLowerCase();
        const filtered: TechCategory[] = [];

        for (const category of this.techStack) {
            const filteredItems = query ? category.items.filter((item) => item.label.toLowerCase().includes(query)) : category.items;

            if (filteredItems.length) {
                filtered.push({ ...category, items: filteredItems });
            }
        }

        this.filteredGroups = filtered;
    }
}
```

## ifta-label-doc

IftaLabel is used to create infield top aligned labels. Visit IftaLabel documentation for more information.

```typescript
import { Component } from '@angular/core';
import { FormsModule } from '@angular/forms';
import { AutoCompleteModule } from 'primeng/autocomplete';
import { IftaLabelModule } from 'primeng/iftalabel';

@Component({
    template: `
        <div class="flex justify-center">
            <p-iftalabel>
                <p-autocomplete [(ngModel)]="value" [suggestions]="items" (completeMethod)="search($event)" inputId="ac" inputStyleClass="w-full md:w-56" scrollHeight="14rem" />
                <label for="ac">Identifier</label>
            </p-iftalabel>
        </div>
    `,
    standalone: true,
    imports: [AutoCompleteModule, IftaLabelModule, FormsModule]
})
export class AutoCompleteIftaLabelDemo {
    items: any[] | undefined;

    value: any;

    search(event: AutoCompleteCompleteEvent) {
        this.items = [...Array(10).keys()].map((item) => event.query + '-' + item);
    }
}
```

## Invalid

The invalid state is applied using the ⁠invalid property to indicate failed validation, which can be integrated with Angular Forms.

```typescript
import { Component } from '@angular/core';
import { FormsModule } from '@angular/forms';
import { AutoCompleteModule } from 'primeng/autocomplete';

@Component({
    template: `
        <div class="flex flex-wrap justify-center gap-4">
            <p-autocomplete [(ngModel)]="value1" [suggestions]="suggestions" [invalid]="!value1" (completeMethod)="search($event)" placeholder="Search" inputStyleClass="w-full md:w-56" scrollHeight="14rem" />
            <p-autocomplete [(ngModel)]="value2" [suggestions]="suggestions" [invalid]="!value2" (completeMethod)="search($event)" variant="filled" placeholder="Search" inputStyleClass="w-full md:w-56" scrollHeight="14rem" />
        </div>
    `,
    standalone: true,
    imports: [AutoCompleteModule, FormsModule]
})
export class AutoCompleteInvalidDemo {
    value1: any;

    value2: any;

    suggestions: any[] | undefined;

    search(event: AutoCompleteCompleteEvent) {
        this.suggestions = [...Array(10).keys()].map((item) => event.query + '-' + item);
    }
}
```

## Loading

Compose a loading indicator alongside the autocomplete input using InputGroup .

```typescript
import { Component } from '@angular/core';
import { FormsModule } from '@angular/forms';
import { AutoCompleteModule } from 'primeng/autocomplete';
import { InputGroupModule } from 'primeng/inputgroup';
import { InputGroupAddonModule } from 'primeng/inputgroupaddon';
import { Spinner } from '@primeicons/angular/spinner';

@Component({
    template: `
        <div class="flex justify-center">
            <div class="w-full md:w-56">
                <p-inputgroup>
                    <p-autocomplete [(ngModel)]="value" [suggestions]="items" (completeMethod)="search($event)" scrollHeight="14rem" />
                    <p-inputgroup-addon>
                        <svg data-p-icon="spinner" class="animate-spin" />
                    </p-inputgroup-addon>
                </p-inputgroup>
            </div>
        </div>
    `,
    standalone: true,
    imports: [AutoCompleteModule, InputGroupModule, InputGroupAddonModule, FormsModule, Spinner]
})
export class AutoCompleteLoadingDemo {
    items: string[] = [];

    value: any;

    search(event: AutoCompleteCompleteEvent) {
        this.items = [...Array(10).keys()].map((item) => event.query + '-' + item);
    }
}
```

## Objects

AutoComplete can also work with objects using the optionLabel property that defines the label to display as a suggestion. The value passed to the model would still be the object instance of a suggestion. Here is an example with a Country object that has name and code fields such as &#123;name: "United States", code:"USA"&#125; .

```typescript
import { Component, OnInit, inject } from '@angular/core';
import { FormsModule } from '@angular/forms';
import { AutoCompleteModule } from 'primeng/autocomplete';
import { CountryService } from '@/service/countryservice';
import { Country } from '@/domain/customer';

@Component({
    template: `
        <div class="flex justify-center">
            <p-autocomplete [(ngModel)]="selectedCountry" [suggestions]="filteredCountries" (completeMethod)="filterCountry($event)" placeholder="Search a country..." optionLabel="name" inputStyleClass="w-full md:w-56" scrollHeight="14rem" />
        </div>
    `,
    standalone: true,
    imports: [AutoCompleteModule, FormsModule],
    providers: [CountryService]
})
export class AutoCompleteObjectsDemo implements OnInit {
    private countryService = inject(CountryService);

    countries: any[] | undefined;

    selectedCountry: any;

    filteredCountries: any[] | undefined;

    ngOnInit() {
        this.countryService.getCountries().then((countries) => {
            this.countries = countries;
        });
    }

    filterCountry(event: AutoCompleteCompleteEvent) {
        let filtered: any[] = [];
        let query = event.query;

        for (let i = 0; i < (this.countries as any[]).length; i++) {
            let country = (this.countries as any[])[i];

            if (country.name.toLowerCase().indexOf(query.toLowerCase()) == 0) {
                filtered.push(country);
            }
        }

        this.filteredCountries = filtered;
    }
}
```

## preview-doc

```typescript
import { Component } from '@angular/core';
import { FormsModule } from '@angular/forms';
import { AutoCompleteModule } from 'primeng/autocomplete';
import { LabelModule } from 'primeng/label';

@Component({
    template: `
        <div class="flex justify-center">
            <div class="flex flex-col">
                <label pLabel class="mb-2">Search tutorials</label>
                <p-autocomplete [(ngModel)]="value" [suggestions]="filteredItems" (completeMethod)="search($event)" placeholder="Search..." inputStyleClass="w-full md:w-72" scrollHeight="14rem" />
                <small class="text-surface-500 mt-1.5 opacity-75">Type <em class="underline">angular</em> to see suggestions</small>
            </div>
        </div>
    `,
    standalone: true,
    imports: [AutoCompleteModule, LabelModule, FormsModule]
})
export class AutoCompletePreviewDemo {
    value: string | undefined;

    filteredItems: string[] = [];

    items: string[] = [
        'angular signals tutorial',
        'angular vs react 2026',
        'angular native crash course',
        'angular v22 new features',
        'angular testing library guide',
        'angular router v22 tutorial',
        'angular performance optimization',
        'angular lazy loading modules',
        'angular form validation',
        'angular design patterns',
        'angular authentication tutorial',
        'angular with typescript beginner',
        'angular animation libraries',
        'angular deploy to production'
    ];

    search(event: AutoCompleteCompleteEvent) {
        const query = event.query.toLowerCase();

        this.filteredItems = query ? this.items.filter((item) => item.toLowerCase().includes(query)) : [...this.items];
    }
}
```

## reactive-forms-doc

```typescript
import { Component, inject } from '@angular/core';
import { ReactiveFormsModule } from '@angular/forms';
import { AutoCompleteModule } from 'primeng/autocomplete';
import { MessageModule } from 'primeng/message';
import { ToastModule } from 'primeng/toast';
import { ButtonModule } from 'primeng/button';
import { MessageService } from 'primeng/api';

interface AutoCompleteCompleteEvent {
    originalEvent: Event;
    query: string;
}

@Component({
    template: `
        <p-toast />
        <div class="flex justify-center">
            <form [formGroup]="exampleForm" (ngSubmit)="onSubmit()" class="flex justify-center flex-col gap-4 md:w-56">
                <div class="flex flex-col gap-1">
                    <p-autocomplete formControlName="value" [suggestions]="items" [invalid]="isInvalid('value')" (completeMethod)="search($event)" fluid />
                    @if (isInvalid('value')) {
                        <p-message severity="error" size="small" variant="simple">Value is required.</p-message>
                    }
                </div>
                <button pButton severity="secondary" type="submit">Submit</button>
            </form>
        </div>
    `,
    standalone: true,
    imports: [AutoCompleteModule, MessageModule, ToastModule, ButtonModule, ReactiveFormsModule]
})
export class AutoCompleteReactiveFormsDemo {
    private fb = inject(FormBuilder);

    messageService = inject(MessageService);

    items: any[] | undefined;

    exampleForm: FormGroup = this.fb.group({
        value: ['', Validators.required]
    });

    formSubmitted: boolean = false;

    search(event: AutoCompleteCompleteEvent) {
        this.items = [...Array(10).keys()].map((item) => event.query + '-' + item);
    }

    onSubmit() {
        this.formSubmitted = true;

        if (this.exampleForm.valid) {
            this.messageService.add({ severity: 'success', summary: 'Success', detail: 'Form is submitted', life: 3000 });
        }
    }

    isInvalid(controlName: string) {
        const control = this.exampleForm.get(controlName);

        return control?.invalid && (control.touched || this.formSubmitted);
    }
}
```

## signal-forms-doc

```typescript
import { Component, inject, signal } from '@angular/core';
import { form, FormField, required, submit } from '@angular/forms/signals';
import { AutoCompleteModule } from 'primeng/autocomplete';
import { MessageModule } from 'primeng/message';
import { ToastModule } from 'primeng/toast';
import { ButtonModule } from 'primeng/button';
import { MessageService } from 'primeng/api';

@Component({
    template: `
        <p-toast />
        <div class="flex justify-center">
            <form novalidate (submit)="onSubmit($event)" class="flex justify-center flex-col gap-4 md:w-56">
                <div class="flex flex-col gap-1">
                    <p-autocomplete [formField]="exampleForm.value" [suggestions]="items" (completeMethod)="search($event)" fluid />
                    @if (exampleForm.value().touched() && exampleForm.value().invalid()) {
                        @for (error of exampleForm.value().errors(); track error.kind) {
                            <p-message severity="error" size="small" variant="simple">{{ error.message }}</p-message>
                        }
                    }
                </div>
                <button pButton severity="secondary" type="submit">Submit</button>
            </form>
        </div>
    `,
    standalone: true,
    imports: [AutoCompleteModule, MessageModule, ToastModule, ButtonModule, FormField]
})
export class AutoCompleteSignalFormsDemo {
    messageService = inject(MessageService);

    items: string[] = [];

    model = signal({ value: '' });

    exampleForm = form(this.model, (path) => {
        required(path.value, {
            when: ({ state }) => state.touched(),
            message: 'Value is required.'
        });
    });

    search(event: AutoCompleteCompleteEvent) {
        this.items = [...Array(10).keys()].map((item) => event.query + '-' + item);
    }

    onSubmit(event: Event) {
        event.preventDefault();
        submit(this.exampleForm, async () => {
            this.messageService.add({ severity: 'success', summary: 'Success', detail: 'Form is submitted', life: 3000 });
        });
    }
}
```

## Sizes

AutoComplete provides small and large sizes as alternatives to the base.

```typescript
import { Component } from '@angular/core';
import { FormsModule } from '@angular/forms';
import { AutoCompleteModule } from 'primeng/autocomplete';

@Component({
    template: `
        <div class="flex flex-col items-center gap-4">
            <p-autocomplete [(ngModel)]="value1" [suggestions]="items" (completeMethod)="search($event)" size="small" placeholder="Small" scrollHeight="14rem" />
            <p-autocomplete [(ngModel)]="value2" [suggestions]="items" (completeMethod)="search($event)" placeholder="Normal" scrollHeight="14rem" />
            <p-autocomplete [(ngModel)]="value3" [suggestions]="items" (completeMethod)="search($event)" size="large" placeholder="Large" scrollHeight="14rem" />
        </div>
    `,
    standalone: true,
    imports: [AutoCompleteModule, FormsModule]
})
export class AutoCompleteSizesDemo {
    items: string[] = [];

    value1: any;

    value2: any;

    value3: any;

    search(event: AutoCompleteCompleteEvent) {
        this.items = [...Array(10).keys()].map((item) => event.query + '-' + item);
    }
}
```

## Template

AutoComplete offers multiple templates for customization through templating.

```typescript
import { Component, OnInit, inject } from '@angular/core';
import { FormsModule } from '@angular/forms';
import { AutoCompleteModule } from 'primeng/autocomplete';
import { ButtonModule } from 'primeng/button';
import { CountryService } from '@/service/countryservice';
import { Plus } from '@primeicons/angular/plus';

@Component({
    template: `
        <div class="flex justify-center">
            <p-autocomplete [(ngModel)]="selectedCountryAdvanced" [suggestions]="filteredCountries" (completeMethod)="filterCountry($event)" optionLabel="name" inputStyleClass="w-full md:w-56" scrollHeight="14rem">
                <ng-template let-country #item>
                    <div class="flex items-center gap-2">
                        <img src="https://primefaces.org/cdn/primeng/images/demo/flag/flag_placeholder.png" [class]="'flag flag-' + country.code.toLowerCase()" style="width: 18px" />
                        <div>{{ country.name }}</div>
                    </div>
                </ng-template>
                <ng-template #header>
                    <div class="font-medium px-3 py-2">Available Countries</div>
                </ng-template>
                <ng-template #footer>
                    <div class="px-3 py-3">
                        <button type="button" pButton fluid severity="secondary" text size="small"><svg data-p-icon="plus" />Add New</button>
                    </div>
                </ng-template>
            </p-autocomplete>
        </div>
    `,
    standalone: true,
    imports: [AutoCompleteModule, ButtonModule, FormsModule, Plus],
    providers: [CountryService]
})
export class AutoCompleteTemplateDemo implements OnInit {
    private countryService = inject(CountryService);

    countries: any[] | undefined;

    selectedCountryAdvanced: any[] | undefined;

    filteredCountries: any[] | undefined;

    ngOnInit() {
        this.countryService.getCountries().then((countries) => {
            this.countries = countries;
        });
    }

    filterCountry(event: AutoCompleteCompleteEvent) {
        let filtered: any[] = [];
        let query = event.query;

        for (let i = 0; i < (this.countries as any[]).length; i++) {
            let country = (this.countries as any[])[i];

            if (country.name.toLowerCase().indexOf(query.toLowerCase()) == 0) {
                filtered.push(country);
            }
        }

        this.filteredCountries = filtered;
    }
}
```

## template-driven-forms-doc

```typescript
import { Component, inject } from '@angular/core';
import { FormsModule } from '@angular/forms';
import { AutoCompleteModule } from 'primeng/autocomplete';
import { MessageModule } from 'primeng/message';
import { ToastModule } from 'primeng/toast';
import { ButtonModule } from 'primeng/button';
import { MessageService } from 'primeng/api';

@Component({
    template: `
        <p-toast />
        <div class="flex justify-center">
            <form #exampleForm="ngForm" (ngSubmit)="onSubmit(exampleForm)" class="flex justify-center flex-col gap-4 md:w-56">
                <div class="flex flex-col gap-1">
                    <p-autocomplete #val="ngModel" [(ngModel)]="value" [suggestions]="items" [invalid]="val.invalid && (val.touched || exampleForm.submitted)" name="val" (completeMethod)="search($event)" required fluid />
                    @if (val.invalid && (val.touched || exampleForm.submitted)) {
                        <p-message severity="error" size="small" variant="simple">Value is required.</p-message>
                    }
                </div>
                <button pButton severity="secondary" type="submit">Submit</button>
            </form>
        </div>
    `,
    standalone: true,
    imports: [AutoCompleteModule, MessageModule, ToastModule, ButtonModule, FormsModule]
})
export class AutoCompleteTemplateDrivenFormsDemo {
    messageService = inject(MessageService);

    items: any[] = [];

    value: any;

    search(event: AutoCompleteCompleteEvent) {
        this.items = [...Array(10).keys()].map((item) => event.query + '-' + item);
    }

    onSubmit(form: any) {
        if (form.valid) {
            this.messageService.add({ severity: 'success', summary: 'Success', detail: 'Form Submitted', life: 3000 });
        }
    }
}
```

## virtual-scroll-doc

Virtual scrolling is an efficient way of rendering the options by displaying a small subset of data in the viewport at any time. When dealing with huge number of options, it is suggested to enable virtual scrolling to avoid performance issues. Usage is simple as setting virtualScroll property to true and defining virtualScrollItemSize to specify the height of an item.

```typescript
import { Component, OnInit } from '@angular/core';
import { FormsModule } from '@angular/forms';
import { AutoCompleteModule } from 'primeng/autocomplete';

interface AutoCompleteCompleteEvent {
    originalEvent: Event;
    query: string;
}

@Component({
    template: `
        <div class="flex justify-center">
            <p-autocomplete [(ngModel)]="selectedItem" [virtualScroll]="true" [suggestions]="filteredItems" [virtualScrollItemSize]="34" (completeMethod)="filterItems($event)" optionLabel="label" [dropdown]="true" />
        </div>
    `,
    standalone: true,
    imports: [AutoCompleteModule, FormsModule]
})
export class AutoCompleteVirtualScrollDemo implements OnInit {
    selectedItem: any;

    filteredItems: any[] | undefined;

    items: any[] | undefined;

    ngOnInit() {
        this.items = [];
        for (let i = 0; i < 10000; i++) {
            this.items.push({ label: 'Item ' + i, value: 'Item ' + i });
        }
    }

    filterItems(event: AutoCompleteCompleteEvent) {
        //in a real application, make a request to a remote url with the query and return filtered results, for demo we filter at client side
        let filtered: any[] = [];
        let query = event.query;

        for (let i = 0; i < (this.items as any[]).length; i++) {
            let item = (this.items as any[])[i];

            if (item.label.toLowerCase().indexOf(query.toLowerCase()) == 0) {
                filtered.push(item);
            }
        }

        this.filteredItems = filtered;
    }
}
```

## Auto Complete

AutoComplete is an input component that provides real-time suggestions when being typed.

### Props

| Name | Type | Default | Description |
|------|------|---------|-------------|
| minQueryLength | number \| undefined | - | Minimum number of characters to initiate a search. |
| delay | number | - | Delay between keystrokes to wait before sending a query. |
| panelStyle | { [klass: string]: any } \| null \| undefined | - | Inline style of the overlay panel element. |
| panelStyleClass | string \| undefined | - | Style class of the overlay panel element. |
| inputStyle | { [klass: string]: any } \| null \| undefined | - | Inline style of the input field. |
| inputId | string \| undefined | - | Identifier of the focus input to match a label defined for the component. |
| inputStyleClass | string \| undefined | - | Inline style of the input field. |
| placeholder | string \| undefined | - | Hint text for the input field. |
| readonly | boolean | - | When present, it specifies that the input cannot be typed. |
| scrollHeight | string | - | Maximum height of the suggestions panel. |
| lazy | boolean | - | Defines if data is loaded and interacted with in lazy manner. |
| virtualScroll | boolean | - | Whether the data should be loaded on demand during scroll. |
| virtualScrollItemSize | number \| undefined | - | Height of an item in the list for VirtualScrolling. |
| virtualScrollOptions | ScrollerOptions \| undefined | - | Whether to use the scroller feature. The properties of scroller component can be used like an object in it. |
| autoHighlight | boolean | - | When enabled, highlights the first item in the list by default. |
| forceSelection | boolean | - | When present, autocomplete clears the manual input if it does not match of the suggestions to force only accepting values from the suggestions. |
| type | string | - | Type of the input, defaults to "text". |
| autoZIndex | boolean | - | Whether to automatically manage layering. |
| baseZIndex | number | - | Base zIndex value to use in layering. |
| ariaLabel | string \| undefined | - | Defines a string that labels the input for accessibility. |
| dropdownAriaLabel | string \| undefined | - | Defines a string that labels the dropdown button for accessibility. |
| ariaLabelledBy | string \| undefined | - | Specifies one or more IDs in the DOM that labels the input field. |
| dropdownIcon | string \| undefined | - | Icon class of the dropdown icon. |
| unique | boolean | - | Ensures uniqueness of selected items on multiple mode. |
| group | boolean | - | Whether to display options as grouped when nested options are provided. |
| completeOnFocus | boolean | - | Whether to run a query when input receives focus. |
| showClear | boolean | - | When enabled, a clear icon is displayed to clear the value. |
| dropdown | boolean | - | Displays a button next to the input field when enabled. |
| showEmptyMessage | boolean | - | Whether to show the empty message or not. |
| dropdownMode | string | - | Specifies the behavior dropdown button. Default "blank" mode sends an empty string and "current" mode sends the input value. |
| multiple | boolean | - | Specifies if multiple values can be selected. **(Deprecated)** |
| addOnTab | boolean | - | When enabled, the input value is added to the selected items on tab key press when multiple is true and typeahead is false. **(Deprecated)** |
| tabindex | number \| undefined | - | Index of the element in tabbing order. |
| dataKey | string \| undefined | - | A property to uniquely identify a value in options. |
| emptyMessage | string \| undefined | - | Text to display when there is no data. Defaults to global value in i18n translation configuration. |
| autofocus | boolean | - | When present, it specifies that the component should automatically get focus on load. |
| autocomplete | string | - | Used to define a string that autocomplete attribute the current element. |
| optionGroupChildren | string | - | Name of the options field of an option group. |
| optionGroupLabel | string | - | Name of the label field of an option group. |
| overlayOptions | OverlayOptions \| undefined | - | Options for the overlay element. |
| suggestions | any[] | - | An array of suggestions to display. |
| optionLabel | string \| ((item: any) => string) \| undefined | - | Property name or getter function to use as the label of an option. |
| optionValue | string \| ((item: any) => string) \| undefined | - | Property name or getter function to use as the value of an option. |
| id | string \| undefined | - | Unique identifier of the component. |
| searchMessage | string \| undefined | '{0} results are available' | Text to display when the search is active. Defaults to global value in i18n translation configuration. |
| emptySelectionMessage | string \| undefined | 'No selected item' | Text to display when filtering does not return any results. Defaults to global value in i18n translation configuration. |
| selectionMessage | string \| undefined | '{0} items selected' | Text to be displayed in hidden accessible field when options are selected. Defaults to global value in i18n translation configuration. |
| autoOptionFocus | boolean | - | Whether to focus on the first visible or selected element when the overlay panel is shown. |
| selectOnFocus | boolean | - | When enabled, the focused option is selected. |
| searchLocale | string \| string[] \| undefined | - | Locale to use in searching. The default locale is the host environment's current locale. |
| optionDisabled | string \| ((item: any) => boolean) \| undefined | - | Property name or getter function to use as the disabled flag of an option, defaults to false when not defined. |
| focusOnHover | boolean | - | When enabled, the hovered option will be focused. |
| typeahead | boolean | true | Whether typeahead is active or not. |
| addOnBlur | boolean | false | Whether to add an item on blur event if the input has value and typeahead is false with multiple mode. **(Deprecated)** |
| separator | string \| RegExp \| undefined | - | Separator char to add item when typeahead is false and multiple mode is enabled. **(Deprecated)** |
| appendTo | HTMLElement \| ElementRef \| TemplateRef<any> \| "self" \| "body" \| null \| undefined | 'self' | Target element to attach the overlay, valid values are "body" or a local ng-template variable of another element (note: use binding with brackets for template variables, e.g. [appendTo]="mydiv" for a div element having #mydiv as variable name). |
| motionOptions | MotionOptions \| undefined | - | The motion options. |
| dt | object \| undefined | undefined | Defines scoped design tokens of the component. |
| unstyled | boolean \| undefined | undefined | Indicates whether the component should be rendered without styles. |
| pt | PassThrough<I, AutoCompletePassThroughOptions<I>> | undefined | Used to pass attributes to DOM elements inside the component. |
| ptOptions | PassThroughOptions \| undefined | undefined | Used to configure passthrough(pt) options of the component. |
| required | boolean \| undefined | false | There must be a value (if set). |
| invalid | boolean \| undefined | false | When present, it specifies that the component should have invalid state style. |
| disabled | boolean \| undefined | false | When present, it specifies that the component should have disabled state style. |
| name | string \| undefined | undefined | When present, it specifies that the name of the input. |
| fluid | boolean \| undefined | false | Spans 100% width of the container when enabled. |
| variant | "filled" \| "outlined" \| undefined | 'outlined' | Specifies the input variant of the component. |
| size | "small" \| "large" \| undefined | undefined | Specifies the size of the component. |
| inputSize | number \| null \| undefined | undefined | Specifies the visible width of the input element in characters. |
| pattern | string \| readonly RegExp[] \| null \| undefined | undefined | Specifies the value must match the pattern. |
| min | any | undefined | The value must be greater than or equal to the value. |
| max | any | undefined | The value must be less than or equal to the value. |
| step | number \| null \| undefined | undefined | Unless the step is set to the any literal, the value must be min + an integral multiple of the step. |
| minlength | number \| null \| undefined | undefined | The number of characters (code points) must not be less than the value of the attribute, if non-empty. |
| maxlength | number \| null \| undefined | undefined | The number of characters (code points) must not exceed the value of the attribute. |

### Emits

| Name | Parameters | Description |
|------|------------|-------------|
| completeMethod | event: AutoCompleteCompleteEvent | Callback to invoke to search for suggestions. |
| onSelect | event: AutoCompleteSelectEvent | Callback to invoke when a suggestion is selected. |
| onUnselect | event: AutoCompleteUnselectEvent | Callback to invoke when a selected value is removed. |
| onAdd | event: AutoCompleteAddEvent | Callback to invoke when an item is added via addOnBlur or separator features. |
| onFocus | event: Event | Callback to invoke when the component receives focus. |
| onBlur | event: Event | Callback to invoke when the component loses focus. |
| onDropdownClick | event: AutoCompleteDropdownClickEvent | Callback to invoke to when dropdown button is clicked. |
| onClear | value: void | Callback to invoke when clear button is clicked. |
| onInputKeydown | event: KeyboardEvent | Callback to invoke on input key down. |
| onKeyUp | event: KeyboardEvent | Callback to invoke on input key up. |
| onShow | value: void | Callback to invoke on overlay is shown. |
| onHide | value: void | Callback to invoke on overlay is hidden. |
| onLazyLoad | event: AutoCompleteLazyLoadEvent | Callback to invoke on lazy load data. |

### Templates

| Name | Type | Description |
|------|------|-------------|
| item | TemplateRef<AutoCompleteItemTemplateContext<any>> \| undefined | Custom item template. |
| empty | TemplateRef<void> \| undefined | Custom empty message template. |
| header | TemplateRef<void> \| undefined | Custom header template. |
| footer | TemplateRef<void> \| undefined | Custom footer template. |
| selecteditem | TemplateRef<AutoCompleteSelectedItemTemplateContext<any>> \| undefined | Custom selected item template. |
| group | TemplateRef<AutoCompleteGroupTemplateContext<any>> \| undefined | Custom group template. |
| loader | TemplateRef<AutoCompleteLoaderTemplateContext> \| undefined | Custom loader template. |
| removeicon | TemplateRef<AutoCompleteRemoveIconTemplateContext> \| undefined | Custom remove icon template. |
| loadingicon | TemplateRef<void> \| undefined | Custom loading icon template. |
| clearicon | TemplateRef<void> \| undefined | Custom clear icon template. |
| dropdownicon | TemplateRef<void> \| undefined | Custom dropdown icon template. |

## Pass Through Options

| Name | Type | Description |
|------|------|-------------|
| host | PassThroughOption<HTMLElement, I> | Used to pass attributes to the host's DOM element. |
| root | PassThroughOption<HTMLElement, I> | Used to pass attributes to the root's DOM element. |
| pcInputText | InputTextPassThrough | Used to pass attributes to the InputText component. |
| inputMultiple | PassThroughOption<HTMLUListElement, I> | Used to pass attributes to the input multiple's DOM element. |
| chipItem | PassThroughOption<HTMLLIElement, I> | Used to pass attributes to the chip item's DOM element. |
| pcChip | ChipPassThrough | Used to pass attributes to the Chip component. |
| chipIcon | PassThroughOption<HTMLSpanElement, I> | Used to pass attributes to the chip icon's DOM element. |
| inputChip | PassThroughOption<HTMLLIElement, I> | Used to pass attributes to the input chip's DOM element. |
| clearIcon | PassThroughOption<SVGElement, I> | Used to pass attributes to the clear icon's DOM element. |
| loader | PassThroughOption<SVGElement, I> | Used to pass attributes to the loader's DOM element. |
| dropdown | PassThroughOption<HTMLButtonElement, I> | Used to pass attributes to the dropdown button's DOM element. |
| pcOverlay | OverlayPassThrough | Used to pass attributes to the Overlay component. |
| overlay | PassThroughOption<HTMLDivElement, I> | Used to pass attributes to the overlay's DOM element. |
| listContainer | PassThroughOption<HTMLDivElement, I> | Used to pass attributes to the list container's DOM element. |
| pcScroller | VirtualScrollerPassThrough | Used to pass attributes to the Scroller component. |
| list | PassThroughOption<HTMLUListElement, I> | Used to pass attributes to the list's DOM element. |
| optionGroup | PassThroughOption<HTMLLIElement, I> | Used to pass attributes to the option group's DOM element. |
| option | PassThroughOption<HTMLLIElement, I> | Used to pass attributes to the option's DOM element. |
| emptyMessage | PassThroughOption<HTMLLIElement, I> | Used to pass attributes to the empty message's DOM element. |

## Theming

### CSS Classes

| Class | Description |
|-------|-------------|
| p-autocomplete | Class name of the root element |
| p-autocomplete-input | Class name of the input element |
| p-autocomplete-input-multiple | Class name of the input multiple element |
| p-autocomplete-chip-item | Class name of the chip item element |
| p-autocomplete-chip | Class name of the chip element |
| p-autocomplete-chip-icon | Class name of the chip icon element |
| p-autocomplete-input-chip | Class name of the input chip element |
| p-autocomplete-loader | Class name of the loader element |
| p-autocomplete-dropdown | Class name of the dropdown element |
| p-autocomplete-overlay | Class name of the panel element |
| p-autocomplete-list | Class name of the list element |
| p-autocomplete-option-group | Class name of the option group element |
| p-autocomplete-option | Class name of the option element |
| p-autocomplete-empty-message | Class name of the empty message element |
| p-autocomplete-clear-icon | Class name of the clear icon |

### Design Tokens

| Token | CSS Variable | Description |
|-------|--------------|-------------|
| autocomplete.background | --p-autocomplete-background | Background of root |
| autocomplete.disabled.background | --p-autocomplete-disabled-background | Disabled background of root |
| autocomplete.filled.background | --p-autocomplete-filled-background | Filled background of root |
| autocomplete.filled.hover.background | --p-autocomplete-filled-hover-background | Filled hover background of root |
| autocomplete.filled.focus.background | --p-autocomplete-filled-focus-background | Filled focus background of root |
| autocomplete.border.color | --p-autocomplete-border-color | Border color of root |
| autocomplete.hover.border.color | --p-autocomplete-hover-border-color | Hover border color of root |
| autocomplete.focus.border.color | --p-autocomplete-focus-border-color | Focus border color of root |
| autocomplete.invalid.border.color | --p-autocomplete-invalid-border-color | Invalid border color of root |
| autocomplete.color | --p-autocomplete-color | Color of root |
| autocomplete.disabled.color | --p-autocomplete-disabled-color | Disabled color of root |
| autocomplete.placeholder.color | --p-autocomplete-placeholder-color | Placeholder color of root |
| autocomplete.invalid.placeholder.color | --p-autocomplete-invalid-placeholder-color | Invalid placeholder color of root |
| autocomplete.shadow | --p-autocomplete-shadow | Shadow of root |
| autocomplete.padding.x | --p-autocomplete-padding-x | Padding x of root |
| autocomplete.padding.y | --p-autocomplete-padding-y | Padding y of root |
| autocomplete.border.radius | --p-autocomplete-border-radius | Border radius of root |
| autocomplete.focus.ring.width | --p-autocomplete-focus-ring-width | Focus ring width of root |
| autocomplete.focus.ring.style | --p-autocomplete-focus-ring-style | Focus ring style of root |
| autocomplete.focus.ring.color | --p-autocomplete-focus-ring-color | Focus ring color of root |
| autocomplete.focus.ring.offset | --p-autocomplete-focus-ring-offset | Focus ring offset of root |
| autocomplete.focus.ring.shadow | --p-autocomplete-focus-ring-shadow | Focus ring shadow of root |
| autocomplete.transition.duration | --p-autocomplete-transition-duration | Transition duration of root |
| autocomplete.overlay.background | --p-autocomplete-overlay-background | Background of overlay |
| autocomplete.overlay.border.color | --p-autocomplete-overlay-border-color | Border color of overlay |
| autocomplete.overlay.border.radius | --p-autocomplete-overlay-border-radius | Border radius of overlay |
| autocomplete.overlay.color | --p-autocomplete-overlay-color | Color of overlay |
| autocomplete.overlay.shadow | --p-autocomplete-overlay-shadow | Shadow of overlay |
| autocomplete.list.padding | --p-autocomplete-list-padding | Padding of list |
| autocomplete.list.gap | --p-autocomplete-list-gap | Gap of list |
| autocomplete.option.focus.background | --p-autocomplete-option-focus-background | Focus background of option |
| autocomplete.option.selected.background | --p-autocomplete-option-selected-background | Selected background of option |
| autocomplete.option.selected.focus.background | --p-autocomplete-option-selected-focus-background | Selected focus background of option |
| autocomplete.option.color | --p-autocomplete-option-color | Color of option |
| autocomplete.option.focus.color | --p-autocomplete-option-focus-color | Focus color of option |
| autocomplete.option.selected.color | --p-autocomplete-option-selected-color | Selected color of option |
| autocomplete.option.selected.focus.color | --p-autocomplete-option-selected-focus-color | Selected focus color of option |
| autocomplete.option.padding | --p-autocomplete-option-padding | Padding of option |
| autocomplete.option.border.radius | --p-autocomplete-option-border-radius | Border radius of option |
| autocomplete.option.font.weight | --p-autocomplete-option-font-weight | Font weight of option |
| autocomplete.option.font.size | --p-autocomplete-option-font-size | Font size of option |
| autocomplete.option.group.background | --p-autocomplete-option-group-background | Background of option group |
| autocomplete.option.group.color | --p-autocomplete-option-group-color | Color of option group |
| autocomplete.option.group.font.weight | --p-autocomplete-option-group-font-weight | Font weight of option group |
| autocomplete.option.group.font.size | --p-autocomplete-option-group-font-size | Font size of option group |
| autocomplete.option.group.padding | --p-autocomplete-option-group-padding | Padding of option group |
| autocomplete.dropdown.width | --p-autocomplete-dropdown-width | Width of dropdown |
| autocomplete.dropdown.sm.width | --p-autocomplete-dropdown-sm-width | Sm width of dropdown |
| autocomplete.dropdown.lg.width | --p-autocomplete-dropdown-lg-width | Lg width of dropdown |
| autocomplete.dropdown.border.color | --p-autocomplete-dropdown-border-color | Border color of dropdown |
| autocomplete.dropdown.hover.border.color | --p-autocomplete-dropdown-hover-border-color | Hover border color of dropdown |
| autocomplete.dropdown.active.border.color | --p-autocomplete-dropdown-active-border-color | Active border color of dropdown |
| autocomplete.dropdown.border.radius | --p-autocomplete-dropdown-border-radius | Border radius of dropdown |
| autocomplete.dropdown.focus.ring.width | --p-autocomplete-dropdown-focus-ring-width | Focus ring width of dropdown |
| autocomplete.dropdown.focus.ring.style | --p-autocomplete-dropdown-focus-ring-style | Focus ring style of dropdown |
| autocomplete.dropdown.focus.ring.color | --p-autocomplete-dropdown-focus-ring-color | Focus ring color of dropdown |
| autocomplete.dropdown.focus.ring.offset | --p-autocomplete-dropdown-focus-ring-offset | Focus ring offset of dropdown |
| autocomplete.dropdown.focus.ring.shadow | --p-autocomplete-dropdown-focus-ring-shadow | Focus ring shadow of dropdown |
| autocomplete.dropdown.background | --p-autocomplete-dropdown-background | Background of dropdown |
| autocomplete.dropdown.hover.background | --p-autocomplete-dropdown-hover-background | Hover background of dropdown |
| autocomplete.dropdown.active.background | --p-autocomplete-dropdown-active-background | Active background of dropdown |
| autocomplete.dropdown.color | --p-autocomplete-dropdown-color | Color of dropdown |
| autocomplete.dropdown.hover.color | --p-autocomplete-dropdown-hover-color | Hover color of dropdown |
| autocomplete.dropdown.active.color | --p-autocomplete-dropdown-active-color | Active color of dropdown |
| autocomplete.chip.border.radius | --p-autocomplete-chip-border-radius | Border radius of chip |
| autocomplete.chip.focus.background | --p-autocomplete-chip-focus-background | Focus background of chip |
| autocomplete.chip.focus.color | --p-autocomplete-chip-focus-color | Focus color of chip |
| autocomplete.empty.message.padding | --p-autocomplete-empty-message-padding | Padding of empty message |
