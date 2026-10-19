---
title: "Angular PickList Component"
type: "component"
version: "latest"
source_url: "https://primeng.dev/llms/components/picklist.md"
description: "PickList is used to reorder items between different lists."
---

# Angular PickList Component

PickList is used to reorder items between different lists.

## Accessibility

Screen Reader Value to describe the source listbox and target listbox can be provided with ariaLabelledBy or ariaLabel props. The list elements has a listbox role with the aria-multiselectable attribute. Each list item has an option role with aria-selected and aria-disabled as their attributes. Controls buttons are button elements with an aria-label that refers to the aria.moveTop , aria.moveUp , aria.moveDown , aria.moveBottom , aria.moveTo , aria.moveAllTo , aria.moveFrom and aria.moveAllFrom properties of the locale API by default, alternatively you may use moveTopButtonProps , moveUpButtonProps , moveDownButtonProps , moveToButtonProps , moveAllToButtonProps , moveFromButtonProps , moveFromButtonProps and moveAllFromButtonProps to customize the buttons like overriding the default aria-label attributes. PickList Keyboard Support Key Function tab Moves focus to the first selected option, if there is none then first option receives the focus. up arrow Moves focus to the previous option. down arrow Moves focus to the next option. enter Toggles the selected state of the focused option. space Toggles the selected state of the focused option. home Moves focus to the first option. end Moves focus to the last option. shift + down arrow Moves focus to the next option and toggles the selection state. shift + up arrow Moves focus to the previous option and toggles the selection state. shift + space Selects the items between the most recently selected option and the focused option. control + shift + home Selects the focused options and all the options up to the first one. control + shift + end Selects the focused options and all the options down to the first one. control + a Selects all options. Buttons Keyboard Support Key Function enter Executes button action. space Executes button action.

## Basic

PickList is used as a controlled input with source and target properties. Content of a list item needs to be defined with the item template that receives an object in the list as parameter. Drag & drop functionality depends on &#64;angular/cdk package.

```typescript
import { Component } from '@angular/core';
import { PickListModule } from 'primeng/picklist';
import { LabelModule } from 'primeng/label';

@Component({
    template: `
        <p-picklist [source]="sourcePeople" [target]="targetPeople" [responsive]="true" breakpoint="1400px">
            <ng-template #sourceHeader>
                <label pLabel class="ml-1.5">Available Members</label>
            </ng-template>
            <ng-template #targetHeader>
                <label pLabel class="ml-1.5">Team Members</label>
            </ng-template>
            <ng-template let-item #item>
                {{ item.name }}
            </ng-template>
            <ng-template #emptymessagesource>
                <div class="flex items-center justify-center min-h-40 p-6 text-surface-500 dark:text-surface-400">No available members</div>
            </ng-template>
            <ng-template #emptymessagetarget>
                <div class="flex items-center justify-center min-h-40 p-6 text-surface-500 dark:text-surface-400">Drag members here</div>
            </ng-template>
        </p-picklist>
    `,
    standalone: true,
    imports: [PickListModule, LabelModule]
})
export class PickListBasicDemo {
    sourcePeople = [
        { name: 'Amy Elsner' },
        { name: 'Asiya Javayant' },
        { name: 'Onyama Limba' },
        { name: 'Anna Fali' },
        { name: 'Bernardo Dominic' },
        { name: 'Elwin Sharvill' },
        { name: 'Ioni Bowcher' },
        { name: 'Stephen Shaw' },
        { name: 'Ivan Magalhaes' },
        { name: 'Xuxue Feng' }
    ];
}
```

## Checkbox

Combine with Checkbox and a select-all header for batch selection alongside drag-and-drop.

```typescript
import { Component, viewChild } from '@angular/core';
import { FormsModule } from '@angular/forms';
import { CheckboxModule } from 'primeng/checkbox';
import { PickListModule } from 'primeng/picklist';
import { LabelModule } from 'primeng/label';

interface Person {
    name: string;
}

@Component({
    template: `
        <p-picklist #pl [source]="sourcePeople" [target]="targetPeople" [dragdrop]="true" [responsive]="true" breakpoint="1400px">
            <ng-template #sourceHeader>
                <div class="flex flex-col -mx-2 -mb-0.5">
                    <div class="px-2 pb-1.5">
                        <label pLabel class="ml-1.5">Available Members</label>
                    </div>
                    <div class="flex items-center gap-2 px-2 py-1" (click)="$event.stopPropagation()">
                        <p-checkbox [binary]="true" [ngModel]="isAllSourceSelected()" [indeterminate]="isSourceIndeterminate()" (onChange)="toggleAllSource($event.checked)" [tabindex]="-1" />
                    </div>
                </div>
            </ng-template>
            <ng-template #targetHeader>
                <div class="flex flex-col -mx-2 -mb-0.5">
                    <div class="px-2 pb-1.5">
                        <label pLabel class="ml-1.5">Team Members</label>
                    </div>
                    <div class="flex items-center gap-2 px-2 py-1" (click)="$event.stopPropagation()">
                        <p-checkbox [binary]="true" [ngModel]="isAllTargetSelected()" [indeterminate]="isTargetIndeterminate()" (onChange)="toggleAllTarget($event.checked)" [tabindex]="-1" />
                    </div>
                </div>
            </ng-template>
            <ng-template let-item #item let-selected="selected">
                <div class="flex items-center gap-2 w-full min-w-0">
                    <p-checkbox [binary]="true" [ngModel]="selected" [readonly]="true" [tabindex]="-1" />
                    <span class="truncate">{{ item.name }}</span>
                </div>
            </ng-template>
            <ng-template #emptymessagesource>
                <div class="flex items-center justify-center min-h-40 p-6 text-surface-500 dark:text-surface-400">No available members</div>
            </ng-template>
            <ng-template #emptymessagetarget>
                <div class="flex items-center justify-center min-h-40 p-6 text-surface-500 dark:text-surface-400">Drag members here</div>
            </ng-template>
        </p-picklist>
    `,
    standalone: true,
    imports: [CheckboxModule, PickListModule, LabelModule, FormsModule]
})
export class PickListCheckboxDemo {
    pickList = viewChild<PickList>('pl');

    sourcePeople: Person[] = [
        { name: 'Amy Elsner' },
        { name: 'Asiya Javayant' },
        { name: 'Onyama Limba' },
        { name: 'Anna Fali' },
        { name: 'Bernardo Dominic' },
        { name: 'Elwin Sharvill' },
        { name: 'Ioni Bowcher' },
        { name: 'Stephen Shaw' },
        { name: 'Ivan Magalhaes' },
        { name: 'Xuxue Feng' }
    ];

    targetPeople: Person[] = [];

    isAllSourceSelected(): boolean {
        const pl = this.pickList();

        return !!pl && this.sourcePeople.length > 0 && pl.selectedItemsSource.length === this.sourcePeople.length;
    }

    isSourceIndeterminate(): boolean {
        const pl = this.pickList();

        return !!pl && pl.selectedItemsSource.length > 0 && pl.selectedItemsSource.length < this.sourcePeople.length;
    }

    isAllTargetSelected(): boolean {
        const pl = this.pickList();

        return !!pl && this.targetPeople.length > 0 && pl.selectedItemsTarget.length === this.targetPeople.length;
    }

    isTargetIndeterminate(): boolean {
        const pl = this.pickList();

        return !!pl && pl.selectedItemsTarget.length > 0 && pl.selectedItemsTarget.length < this.targetPeople.length;
    }

    toggleAllSource(checked: boolean) {
        const pl = this.pickList();

        if (pl) {
            pl.selectedItemsSource = checked ? [...this.sourcePeople] : [];
        }
    }

    toggleAllTarget(checked: boolean) {
        const pl = this.pickList();

        if (pl) {
            pl.selectedItemsTarget = checked ? [...this.targetPeople] : [];
        }
    }
}
```

## dragdrop-doc

Enable drag-and-drop by setting dragdrop to true. Items can be dragged within a list to reorder, or dragged across lists to transfer.

```typescript
import { Component } from '@angular/core';
import { PickListModule } from 'primeng/picklist';
import { LabelModule } from 'primeng/label';

@Component({
    template: `
        <p-picklist [source]="sourcePeople" [target]="targetPeople" [dragdrop]="true" [responsive]="true" breakpoint="1400px">
            <ng-template #sourceHeader>
                <label pLabel class="ml-1.5">Available Members</label>
            </ng-template>
            <ng-template #targetHeader>
                <label pLabel class="ml-1.5">Team Members</label>
            </ng-template>
            <ng-template let-item #item>
                {{ item.name }}
            </ng-template>
            <ng-template #emptymessagesource>
                <div class="flex items-center justify-center min-h-40 p-6 text-surface-500 dark:text-surface-400">No available members</div>
            </ng-template>
            <ng-template #emptymessagetarget>
                <div class="flex items-center justify-center min-h-40 p-6 text-surface-500 dark:text-surface-400">Drag members here</div>
            </ng-template>
        </p-picklist>
    `,
    standalone: true,
    imports: [PickListModule, LabelModule]
})
export class PickListDragDropDemo {
    sourcePeople = [
        { name: 'Amy Elsner' },
        { name: 'Asiya Javayant' },
        { name: 'Onyama Limba' },
        { name: 'Anna Fali' },
        { name: 'Bernardo Dominic' },
        { name: 'Elwin Sharvill' },
        { name: 'Ioni Bowcher' },
        { name: 'Stephen Shaw' },
        { name: 'Ivan Magalhaes' },
        { name: 'Xuxue Feng' }
    ];
}
```

## Filter

Filter value is checked against the property of an object configured with the filterBy property.

```typescript
import { Component, OnInit, inject, signal } from '@angular/core';
import { PickListModule } from 'primeng/picklist';
import { ProductService } from '@/service/productservice';
import { Product } from '@/domain/product';

@Component({
    template: `
        <p-picklist
            [source]="sourceProducts()"
            [target]="targetProducts()"
            [dragdrop]="true"
            [responsive]="true"
            filterBy="name"
            sourceFilterPlaceholder="Search by name"
            targetFilterPlaceholder="Search by name"
            breakpoint="1400px"
            scrollHeight="20rem"
        >
            <ng-template let-option let-selected="selected" #option>
                <div class="flex flex-wrap p-1 items-center gap-4 w-full">
                    <img class="w-12 shrink-0 rounded-sm" src="https://primefaces.org/cdn/primeng/images/demo/product/{{ option.image }}" [alt]="option.name" />
                    <div class="flex-1 flex flex-col">
                        <span class="font-medium text-sm">{{ option.name }}</span>
                        <span [class]="selected ? 'text-sm text-inherit' : 'text-sm text-surface-500 dark:text-surface-400'">{{ option.category }}</span>
                    </div>
                    <span class="font-bold sm:ml-8">{{ '$' + option.price }}</span>
                </div>
            </ng-template>
            <ng-template #emptymessagesource>
                <div class="flex items-center justify-center min-h-40 p-6 text-surface-500 dark:text-surface-400">No products available</div>
            </ng-template>
            <ng-template #emptymessagetarget>
                <div class="flex items-center justify-center min-h-40 p-6 text-surface-500 dark:text-surface-400">Drag products here</div>
            </ng-template>
            <ng-template #emptyfiltermessagesource>
                <div class="flex items-center justify-center min-h-40 p-6 text-surface-500 dark:text-surface-400">No results found</div>
            </ng-template>
            <ng-template #emptyfiltermessagetarget>
                <div class="flex items-center justify-center min-h-40 p-6 text-surface-500 dark:text-surface-400">No results found</div>
            </ng-template>
        </p-picklist>
    `,
    standalone: true,
    imports: [PickListModule],
    providers: [ProductService]
})
export class PickListFilterDemo implements OnInit {
    private carService = inject(ProductService);

    sourceProducts = signal<Product[]>([]);

    targetProducts = signal<Product[]>([]);

    ngOnInit() {
        this.carService.getProductsSmall().then((products) => {
            this.sourceProducts.set(products);
        });
    }
}
```

## preview-doc

```typescript
import { Component } from '@angular/core';
import { AvatarModule } from 'primeng/avatar';
import { PickListModule } from 'primeng/picklist';
import { LabelModule } from 'primeng/label';
import { Product } from '@/domain/product';

@Component({
    template: `
        <p-picklist [source]="sourceMembers" [target]="targetMembers" [dragdrop]="true" [responsive]="true" breakpoint="1400px">
            <ng-template #sourceHeader>
                <label pLabel class="ml-1.5">Available Members</label>
            </ng-template>
            <ng-template #targetHeader>
                <label pLabel class="ml-1.5">Team Members</label>
            </ng-template>
            <ng-template let-item #item>
                <div class="flex items-center gap-3 w-full min-w-0">
                    <p-avatar [image]="item.avatar" shape="circle" />
                    <div class="flex flex-col min-w-0">
                        <span class="text-sm font-medium truncate">{{ item.name }}</span>
                        <span class="text-xs opacity-70 truncate">{{ item.role }}</span>
                    </div>
                </div>
            </ng-template>
            <ng-template #emptymessagesource>
                <div class="flex items-center justify-center min-h-40 p-6 text-surface-500 dark:text-surface-400">No available members</div>
            </ng-template>
            <ng-template #emptymessagetarget>
                <div class="flex items-center justify-center min-h-40 p-6 text-surface-500 dark:text-surface-400">Drag members here</div>
            </ng-template>
        </p-picklist>
    `,
    standalone: true,
    imports: [AvatarModule, PickListModule, LabelModule]
})
export class PickListPreviewDemo {
    sourceMembers = [
        { name: 'Amy Elsner', role: 'Designer', avatar: 'https://primefaces.org/cdn/primevue/images/avatar/amyelsner.png' },
        { name: 'Asiya Javayant', role: 'Engineer', avatar: 'https://primefaces.org/cdn/primevue/images/avatar/asiyajavayant.png' },
        { name: 'Onyama Limba', role: 'Product Manager', avatar: 'https://primefaces.org/cdn/primevue/images/avatar/onyamalimba.png' },
        { name: 'Anna Fali', role: 'Engineer', avatar: 'https://primefaces.org/cdn/primevue/images/avatar/annafali.png' },
        { name: 'Bernardo Dominic', role: 'Designer', avatar: 'https://primefaces.org/cdn/primevue/images/avatar/bernardodominic.png' },
        { name: 'Elwin Sharvill', role: 'Engineer', avatar: 'https://primefaces.org/cdn/primevue/images/avatar/elwinsharvill.png' },
        { name: 'Ioni Bowcher', role: 'QA', avatar: 'https://primefaces.org/cdn/primevue/images/avatar/ionibowcher.png' },
        { name: 'Stephen Shaw', role: 'Engineer', avatar: 'https://primefaces.org/cdn/primevue/images/avatar/stephenshaw.png' }
    ];
}
```

## Template

For custom content support define an item template that gets the item instance as a parameter. In addition sourceheader and targetheader templates are provided for further customization.

```typescript
import { Component, OnInit, inject, signal } from '@angular/core';
import { PickListModule } from 'primeng/picklist';
import { ProductService } from '@/service/productservice';
import { Product } from '@/domain/product';

@Component({
    template: `
        <p-picklist [source]="sourceProducts()" [target]="targetProducts()" [dragdrop]="true" [responsive]="true" sourceFilterPlaceholder="Search by name" targetFilterPlaceholder="Search by name" breakpoint="1400px" scrollHeight="20rem">
            <ng-template let-option let-selected="selected" #item>
                <div class="flex flex-wrap p-1 items-center gap-4 w-full">
                    <img class="w-12 shrink-0 rounded-sm" src="https://primefaces.org/cdn/primeng/images/demo/product/{{ option.image }}" [alt]="option.name" />
                    <div class="flex-1 flex flex-col">
                        <span class="font-medium text-sm">{{ option.name }}</span>
                        <span [class]="selected ? 'text-sm text-inherit' : 'text-sm text-surface-500 dark:text-surface-400'">{{ option.category }}</span>
                    </div>
                    <span class="font-bold sm:ml-8">{{ '$' + option.price }}</span>
                </div>
            </ng-template>
            <ng-template #emptymessagesource>
                <div class="flex items-center justify-center min-h-40 p-6 text-surface-500 dark:text-surface-400">No products available</div>
            </ng-template>
            <ng-template #emptymessagetarget>
                <div class="flex items-center justify-center min-h-40 p-6 text-surface-500 dark:text-surface-400">Drag products here</div>
            </ng-template>
        </p-picklist>
    `,
    standalone: true,
    imports: [PickListModule],
    providers: [ProductService]
})
export class PickListTemplateDemo implements OnInit {
    private carService = inject(ProductService);

    sourceProducts = signal<Product[]>([]);

    targetProducts = signal<Product[]>([]);

    ngOnInit() {
        this.carService.getProductsSmall().then((products) => {
            this.sourceProducts.set(products);
        });
    }
}
```

## templates-doc

```typescript
import { Component } from '@angular/core';

@Component({
    template: `
        <div class="doc-tablewrapper">
            <table class="doc-table">
                <thead>
                    <tr>
                        <th>Name</th>
                        <th>Parameters</th>
                    </tr>
                </thead>
                <tbody>
                    <tr>
                        <td>item</td>
                        <td>
                            $implicit: Data of the item<br />
                            index: Index of the item
                        </td>
                    </tr>
                    <tr>
                        <td>sourceHeader</td>
                        <td>-</td>
                    </tr>
                    <tr>
                        <td>targetHeader</td>
                        <td>-</td>
                    </tr>
                    <tr>
                        <td>sourceFilter</td>
                        <td>
                            options.filter: Callback to filter data by the value param<br />
                            options.reset: Resets the filters.
                        </td>
                    </tr>
                    <tr>
                        <td>targetFilter</td>
                        <td>
                            options.filter: Callback to filter data by the value param<br />
                            options.reset: Resets the filters
                        </td>
                    </tr>
                    <tr>
                        <td>emptymessagesource</td>
                        <td>-</td>
                    </tr>
                    <tr>
                        <td>emptyfiltermessagesource</td>
                        <td>-</td>
                    </tr>
                    <tr>
                        <td>emptymessagetarget</td>
                        <td>-</td>
                    </tr>
                    <tr>
                        <td>emptyfiltermessagetarget</td>
                        <td>-</td>
                    </tr>
                    <tr>
                        <td>moveupicon</td>
                        <td>-</td>
                    </tr>
                    <tr>
                        <td>movetopicon</td>
                        <td>-</td>
                    </tr>
                    <tr>
                        <td>movedownicon</td>
                        <td>-</td>
                    </tr>
                    <tr>
                        <td>movebottomicon</td>
                        <td>-</td>
                    </tr>
                    <tr>
                        <td>movetotargeticon</td>
                        <td>$implicit: viewChanged</td>
                    </tr>
                    <tr>
                        <td>movealltotargeticon</td>
                        <td>$implicit: viewChanged</td>
                    </tr>
                    <tr>
                        <td>movetosourceicon</td>
                        <td>$implicit: viewChanged</td>
                    </tr>
                    <tr>
                        <td>movealltosourceicon</td>
                        <td>$implicit: viewChanged</td>
                    </tr>
                    <tr>
                        <td>targetfiltericon</td>
                        <td>-</td>
                    </tr>
                    <tr>
                        <td>sourcefiltericon</td>
                        <td>-</td>
                    </tr>
                </tbody>
            </table>
        </div>
    `,
    standalone: true,
    imports: []
})
export class PickListTemplatesDemo {}
```

## Pick List

PickList is used to reorder items between different lists.

### Props

| Name | Type | Default | Description |
|------|------|---------|-------------|
| dt | object \| undefined | undefined | Defines scoped design tokens of the component. |
| unstyled | boolean \| undefined | undefined | Indicates whether the component should be rendered without styles. |
| pt | PassThrough<I, PickListPassThroughOptions<I>> | undefined | Used to pass attributes to DOM elements inside the component. |
| ptOptions | PassThroughOptions \| undefined | undefined | Used to configure passthrough(pt) options of the component. |
| source | any[] | - | An array of objects for the source list. |
| target | any[] | - | An array of objects for the target list. |
| dataKey | string \| undefined | - | Name of the field that uniquely identifies the options. |
| sourceHeader | string \| undefined | - | Text for the source list caption |
| tabindex | number | - | Index of the element in tabbing order. |
| rightButtonAriaLabel | string \| undefined | - | Defines a string that labels the move to right button for accessibility. |
| leftButtonAriaLabel | string \| undefined | - | Defines a string that labels the move to left button for accessibility. |
| allRightButtonAriaLabel | string \| undefined | - | Defines a string that labels the move to all right button for accessibility. |
| allLeftButtonAriaLabel | string \| undefined | - | Defines a string that labels the move to all left button for accessibility. |
| upButtonAriaLabel | string \| undefined | - | Defines a string that labels the move to up button for accessibility. |
| downButtonAriaLabel | string \| undefined | - | Defines a string that labels the move to down button for accessibility. |
| topButtonAriaLabel | string \| undefined | - | Defines a string that labels the move to top button for accessibility. |
| bottomButtonAriaLabel | string \| undefined | - | Defines a string that labels the move to bottom button for accessibility. |
| sourceAriaLabel | string \| undefined | - | Defines a string that labels the source list. |
| targetAriaLabel | string \| undefined | - | Defines a string that labels the target list. |
| targetHeader | string \| undefined | - | Text for the target list caption |
| responsive | boolean | - | When enabled orderlist adjusts its controls based on screen size. |
| filterBy | string \| undefined | - | When specified displays an input field to filter the items on keyup and decides which field to search (Accepts multiple fields with a comma). |
| filterLocale | string \| undefined | - | Locale to use in filtering. The default locale is the host environment's current locale. |
| trackBy | Function | - | Function to optimize the dom operations by delegating to ngForTrackBy, default algorithm checks for object identity. Use sourceTrackBy or targetTrackBy in case different algorithms are needed per list. |
| sourceTrackBy | Function \| undefined | - | Function to optimize the dom operations by delegating to ngForTrackBy in source list, default algorithm checks for object identity. |
| targetTrackBy | Function \| undefined | - | Function to optimize the dom operations by delegating to ngForTrackBy in target list, default algorithm checks for object identity. |
| showSourceFilter | boolean | - | Whether to show filter input for source list when filterBy is enabled. |
| showTargetFilter | boolean | - | Whether to show filter input for target list when filterBy is enabled. |
| metaKeySelection | boolean | - | Defines how multiple items can be selected, when true metaKey needs to be pressed to select or unselect an item and when set to false selection of each item can be toggled individually. On touch enabled devices, metaKeySelection is turned off automatically. |
| dragdrop | boolean | - | Whether to enable dragdrop based reordering. |
| style | { [klass: string]: any } \| null \| undefined | - | Inline style of the component. |
| sourceStyle | { [klass: string]: any } \| null \| undefined | - | Inline style of the source list element. |
| targetStyle | { [klass: string]: any } \| null \| undefined | - | Inline style of the target list element. |
| showSourceControls | boolean | - | Whether to show buttons of source list. |
| showTargetControls | boolean | - | Whether to show buttons of target list. |
| sourceFilterPlaceholder | string \| undefined | - | Placeholder text on source filter input. |
| targetFilterPlaceholder | string \| undefined | - | Placeholder text on target filter input. |
| disabled | boolean | - | When present, it specifies that the component should be disabled. |
| sourceOptionDisabled | string \| ((item: any) => boolean) \| undefined | - | Name of the disabled field of a target option or function to determine disabled state. |
| targetOptionDisabled | string \| ((item: any) => boolean) \| undefined | - | Name of the disabled field of a target option or function to determine disabled state. |
| ariaSourceFilterLabel | string \| undefined | - | Defines a string that labels the filter input of source list. |
| ariaTargetFilterLabel | string \| undefined | - | Defines a string that labels the filter input of target list. |
| filterMatchMode | "startsWith" \| "contains" \| "notContains" \| "endsWith" \| "equals" \| "notEquals" \| "in" \| "between" \| "lt" \| "lte" \| "gt" \| "gte" \| "is" \| "isNot" \| "before" \| "after" \| "dateIs" \| "dateIsNot" \| "dateBefore" \| "dateAfter" | - | Defines how the items are filtered. |
| stripedRows | boolean | - | Whether to displays rows with alternating colors. |
| keepSelection | boolean | - | Keeps selection on the transfer list. |
| scrollHeight | string | - | Height of the viewport, a scrollbar is defined if height of list exceeds this value. |
| autoOptionFocus | boolean | - | Whether to focus on the first visible or selected element. |
| buttonProps | ButtonDirectiveOptions | - | Configuration object forwarded to the Button component. |
| moveUpButtonProps | ButtonDirectiveOptions \| undefined | - | Configuration object forwarded to the move up button inside the component. |
| moveTopButtonProps | ButtonDirectiveOptions \| undefined | - | Configuration object forwarded to the move top button inside the component. |
| moveDownButtonProps | ButtonDirectiveOptions \| undefined | - | Configuration object forwarded to the move down button inside the component. |
| moveBottomButtonProps | ButtonDirectiveOptions \| undefined | - | Configuration object forwarded to the move bottom button inside the component. |
| moveToTargetProps | ButtonDirectiveOptions \| undefined | - | Configuration object forwarded to the move to target button inside the component. |
| moveAllToTargetProps | ButtonDirectiveOptions \| undefined | - | Configuration object forwarded to the move all to target button inside the component. |
| moveToSourceProps | ButtonDirectiveOptions \| undefined | - | Configuration object forwarded to the move to source button inside the component. |
| moveAllToSourceProps | ButtonDirectiveOptions \| undefined | - | Configuration object forwarded to the move all to source button inside the component. |
| breakpoint | string | - | Indicates the width of the screen at which the component should change its behavior. |

### Emits

| Name | Parameters | Description |
|------|------------|-------------|
| onMoveToSource | event: PickListMoveToSourceEvent | Callback to invoke when items are moved from target to source. |
| onMoveAllToSource | event: PickListMoveAllToSourceEvent | Callback to invoke when all items are moved from target to source. |
| onMoveAllToTarget | event: PickListMoveAllToTargetEvent | Callback to invoke when all items are moved from source to target. |
| onMoveToTarget | event: PickListMoveToTargetEvent | Callback to invoke when items are moved from source to target. |
| onSourceReorder | event: PickListSourceReorderEvent | Callback to invoke when items are reordered within source list. |
| onTargetReorder | event: PickListTargetReorderEvent | Callback to invoke when items are reordered within target list. |
| onSourceSelect | event: PickListSourceSelectEvent | Callback to invoke when items are selected within source list. |
| onTargetSelect | event: PickListTargetSelectEvent | Callback to invoke when items are selected within target list. |
| onSourceFilter | event: PickListSourceFilterEvent | Callback to invoke when the source list is filtered |
| onTargetFilter | event: PickListTargetFilterEvent | Callback to invoke when the target list is filtered |
| onFocus | event: Event | Callback to invoke when the list is focused |
| onBlur | event: Event | Callback to invoke when the list is blurred |

### Templates

| Name | Type | Description |
|------|------|-------------|
| item | TemplateRef<PickListItemTemplateContext> \| undefined | Custom item template. |
| sourceitem | TemplateRef<PickListItemTemplateContext> \| undefined | Custom source item template, falls back to the shared item template when not defined. |
| targetitem | TemplateRef<PickListItemTemplateContext> \| undefined | Custom target item template, falls back to the shared item template when not defined. |
| sourceheader | TemplateRef<void> \| undefined | Custom source header template. |
| targetheader | TemplateRef<void> \| undefined | Custom target header template. |
| sourcefilter | TemplateRef<PickListFilterTemplateContext> \| undefined | Custom source filter template. |
| targetfilter | TemplateRef<PickListFilterTemplateContext> \| undefined | Custom target filter template. |
| emptymessagesource | TemplateRef<void> \| undefined | Custom empty message when source is empty template. |
| emptyfiltermessagesource | TemplateRef<void> \| undefined | Custom empty filter message when source is empty template. |
| emptymessagetarget | TemplateRef<void> \| undefined | Custom empty message when target is empty template. |
| emptyfiltermessagetarget | TemplateRef<void> \| undefined | Custom empty filter message when target is empty template. |
| moveupicon | TemplateRef<void> \| undefined | Custom move up icon template. |
| movetopicon | TemplateRef<void> \| undefined | Custom move top icon template. |
| movedownicon | TemplateRef<void> \| undefined | Custom move down icon template. |
| movebottomicon | TemplateRef<void> \| undefined | Custom move bottom icon template. |
| movetotargeticon | TemplateRef<PickListTransferIconTemplateContext> \| undefined | Custom move to target icon template. |
| movealltotargeticon | TemplateRef<PickListTransferIconTemplateContext> \| undefined | Custom move all to target icon template. |
| movetosourceicon | TemplateRef<PickListTransferIconTemplateContext> \| undefined | Custom move to source icon template. |
| movealltosourceicon | TemplateRef<PickListTransferIconTemplateContext> \| undefined | Custom move all to source icon template. |
| targetfiltericon | TemplateRef<void> \| undefined | Custom target filter icon template. |
| sourcefiltericon | TemplateRef<void> \| undefined | Custom source filter icon template. |

## Pass Through Options

| Name | Type | Description |
|------|------|-------------|
| root | PassThroughOption<HTMLDivElement, I> | Used to pass attributes to the root's DOM element. |
| sourceControls | PassThroughOption<HTMLDivElement, I> | Used to pass attributes to the source controls' DOM element. |
| sourceListContainer | PassThroughOption<HTMLDivElement, I> | Used to pass attributes to the source list container's DOM element. |
| transferControls | PassThroughOption<HTMLDivElement, I> | Used to pass attributes to the transfer controls' DOM element. |
| targetListContainer | PassThroughOption<HTMLDivElement, I> | Used to pass attributes to the target list container's DOM element. |
| targetControls | PassThroughOption<HTMLDivElement, I> | Used to pass attributes to the target controls' DOM element. |
| pcSourceMoveUpButton | ButtonPassThrough | Used to pass attributes to the source move up Button component. |
| pcSourceMoveTopButton | ButtonPassThrough | Used to pass attributes to the source move top Button component. |
| pcSourceMoveDownButton | ButtonPassThrough | Used to pass attributes to the source move down Button component. |
| pcSourceMoveBottomButton | ButtonPassThrough | Used to pass attributes to the source move bottom Button component. |
| pcMoveToTargetButton | ButtonPassThrough | Used to pass attributes to the move to target Button component. |
| pcMoveAllToTargetButton | ButtonPassThrough | Used to pass attributes to the move all to target Button component. |
| pcMoveToSourceButton | ButtonPassThrough | Used to pass attributes to the move to source Button component. |
| pcMoveAllToSourceButton | ButtonPassThrough | Used to pass attributes to the move all to source Button component. |
| pcTargetMoveUpButton | ButtonPassThrough | Used to pass attributes to the target move up Button component. |
| pcTargetMoveTopButton | ButtonPassThrough | Used to pass attributes to the target move top Button component. |
| pcTargetMoveDownButton | ButtonPassThrough | Used to pass attributes to the target move down Button component. |
| pcTargetMoveBottomButton | ButtonPassThrough | Used to pass attributes to the target move bottom Button component. |
| pcListbox | ListBoxPassThrough | Used to pass attributes to the Listbox component. |

## Theming

### CSS Classes

| Class | Description |
|-------|-------------|
| p-picklist | Class name of the root element |
| p-picklist-source-controls | Class name of the source controls element |
| p-picklist-source-list-container | Class name of the source list container element |
| p-picklist-transfer-controls | Class name of the transfer controls element |
| p-picklist-target-list-container | Class name of the target list container element |
| p-picklist-target-controls | Class name of the target controls element |

### Design Tokens

| Token | CSS Variable | Description |
|-------|--------------|-------------|
| picklist.gap | --p-picklist-gap | Gap of root |
| picklist.controls.gap | --p-picklist-controls-gap | Gap of controls |
