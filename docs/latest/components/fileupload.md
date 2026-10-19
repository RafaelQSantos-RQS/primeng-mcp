---
title: "Angular FileUpload Component"
type: "component"
version: "latest"
source_url: "https://primeng.dev/llms/components/fileupload.md"
description: "FileUpload is an advanced uploader with dragdrop support, multi file uploads, auto uploading, progress tracking and validations."
---

# Angular FileUpload Component

FileUpload is an advanced uploader with dragdrop support, multi file uploads, auto uploading, progress tracking and validations.

## Accessibility

Screen Reader FileUpload uses a hidden native input element with type="file" for screen readers. Keyboard Support Interactive elements of the uploader are buttons, visit the Button accessibility section for more information.

## Advanced

Advanced uploader provides dragdrop support, multi file uploads, auto uploading, progress tracking and validations.

```typescript
import { Component, inject } from '@angular/core';
import { FileUploadModule } from 'primeng/fileupload';
import { MessageService } from 'primeng/api';

@Component({
    template: `
        <p-fileupload name="demo[]" url="https://www.primefaces.org/cdn/api/upload.php" (onUpload)="onUpload()" [multiple]="true" accept="image/*" maxFileSize="1000000" mode="advanced" [pt]="{ header: { class: 'flex-wrap' } }">
            <ng-template #empty>
                <div>Drag and drop files to here to upload.</div>
            </ng-template>
        </p-fileupload>
    `,
    standalone: true,
    imports: [FileUploadModule],
    providers: [MessageService]
})
export class FileUploadAdvancedDemo {
    private messageService = inject(MessageService);

    onUpload() {
        this.messageService.add({ severity: 'info', summary: 'Success', detail: 'File Uploaded' });
    }
}
```

## Auto

When auto property is enabled, a file gets uploaded instantly after selection.

```typescript
import { Component, inject } from '@angular/core';
import { FileUploadModule } from 'primeng/fileupload';
import { MessageService } from 'primeng/api';

@Component({
    template: `
        <div class="flex justify-center">
            <p-fileupload mode="basic" name="demo[]" url="https://www.primefaces.org/cdn/api/upload.php" accept="image/*" maxFileSize="1000000" (onUpload)="onUpload()" [auto]="true" chooseLabel="Browse" />
        </div>
    `,
    standalone: true,
    imports: [FileUploadModule],
    providers: [MessageService]
})
export class FileUploadAutoDemo {
    private messageService = inject(MessageService);

    onUpload() {
        this.messageService.add({ severity: 'info', summary: 'Success', detail: 'File Uploaded' });
    }
}
```

## Basic

Select and upload files with drag-and-drop support.

```typescript
import { Component, inject } from '@angular/core';
import { FileUploadModule } from 'primeng/fileupload';
import { ButtonModule } from 'primeng/button';
import { MessageService } from 'primeng/api';

@Component({
    template: `
        <div class="flex justify-between">
            <div class="flex flex-wrap items-center gap-3">
                <p-fileupload #fu mode="basic" chooseLabel="Choose" name="demo[]" url="https://www.primefaces.org/cdn/api/upload.php" accept="image/*" [multiple]="true" maxFileSize="1000000" (onUpload)="onUpload()" />
            </div>
            <button type="button" pButton severity="secondary" (click)="fu.upload()">Upload</button>
        </div>
    `,
    standalone: true,
    imports: [FileUploadModule, ButtonModule],
    providers: [MessageService]
})
export class FileUploadBasicDemo {
    private messageService = inject(MessageService);

    onUpload() {
        this.messageService.add({ severity: 'info', summary: 'Success', detail: 'File Uploaded' });
    }
}
```

## Custom Upload

Uploading implementation can be overridden by enabling customUpload property and defining a custom upload handler.

```typescript
import { Component } from '@angular/core';
import { FileUploadModule } from 'primeng/fileupload';

@Component({
    template: `
        <div class="flex flex-col items-center gap-6">
            <p-fileupload mode="basic" [customUpload]="true" [auto]="true" (uploadHandler)="onFileSelect($event)" chooseLabel="Browse" [chooseButtonProps]="{ severity: 'secondary', variant: 'outlined' }" />
            @if (imageSrc) {
                <img [src]="imageSrc" alt="Image" class="shadow-md rounded-xl w-full sm:w-64" style="filter: grayscale(100%)" />
            }
        </div>
    `,
    standalone: true,
    imports: [FileUploadModule]
})
export class FileUploadCustomDemo {
    imageSrc: string | null = null;

    onFileSelect(event: FileUploadHandlerEvent) {
        const file = event.files[0];
        const reader = new FileReader();

        reader.onload = (e) => {
            if (e.target?.result && typeof e.target.result === 'string') {
                this.imageSrc = e.target.result;
            }
        };

        reader.readAsDataURL(file);
    }
}
```

## Dropzone

A custom dropzone can be built with the content and empty templates. Files are added by dragging them onto the area or by clicking to browse, and the selected files are managed with the removeFileCallback .

```typescript
import { Component } from '@angular/core';
import { FileUploadModule } from 'primeng/fileupload';
import { MessageModule } from 'primeng/message';
import { ButtonModule } from 'primeng/button';
import { CloudUpload } from '@primeicons/angular/cloud-upload';
import { Times } from '@primeicons/angular/times';

@Component({
    template: `
        <p-fileupload
            #fu
            name="demo[]"
            url="https://www.primefaces.org/cdn/api/upload.php"
            [multiple]="true"
            accept="image/*"
            [maxFileSize]="1000000"
            mode="advanced"
            [pt]="{ root: { class: 'border-0! bg-transparent!' }, header: { class: 'hidden!' }, content: { class: 'border-2! border-dashed! border-surface-200! dark:border-surface-700! rounded-xl! p-8!' } }"
        >
            <ng-template #content let-files let-removeFileCallback="removeFileCallback" let-messages="messages">
                @if (messages?.length) {
                    <div class="flex flex-col gap-2">
                        @for (msg of messages; track $index) {
                            <p-message [severity]="msg.severity">{{ msg.text }}</p-message>
                        }
                    </div>
                }
                @if (files.length) {
                    <div class="flex flex-col gap-4">
                        <div class="flex items-center justify-between">
                            <span class="text-sm text-muted-color">{{ files.length }} file(s) selected</span>
                            <div class="flex items-center gap-2">
                                <button type="button" pButton variant="text" size="small" (click)="fu.upload()">Upload</button>
                                <button type="button" pButton variant="text" size="small" severity="danger" (click)="fu.clear()">Clear all</button>
                            </div>
                        </div>
                        <div class="flex flex-col gap-2">
                            @for (file of files; track file.name + file.size; let i = $index) {
                                <div class="flex items-center justify-between p-3 rounded-lg bg-surface-50 dark:bg-surface-800">
                                    <div class="flex items-center gap-3">
                                        <svg data-p-icon="cloud-upload" class="text-primary shrink-0" />
                                        <div class="flex flex-col">
                                            <span class="text-sm font-medium">{{ file.name }}</span>
                                            <span class="text-xs text-muted-color">{{ formatSize(file.size) }}</span>
                                        </div>
                                    </div>
                                    <button type="button" pButton iconOnly variant="text" severity="secondary" size="small" rounded (click)="removeFileCallback($event, i)">
                                        <svg data-p-icon="times" />
                                    </button>
                                </div>
                            }
                        </div>
                    </div>
                }
            </ng-template>
            <ng-template #empty>
                <div class="flex flex-col items-center justify-center gap-3 py-8 cursor-pointer" (click)="fu.choose()">
                    <svg data-p-icon="cloud-upload" [size]="48" class="text-muted-color" />
                    <div class="text-center">
                        <p class="text-lg font-medium mt-0 mb-1">Drop files here</p>
                        <p class="text-sm text-muted-color m-0">or click to browse</p>
                    </div>
                </div>
            </ng-template>
        </p-fileupload>
    `,
    standalone: true,
    imports: [FileUploadModule, MessageModule, ButtonModule, CloudUpload, Times]
})
export class FileUploadDropzoneDemo {
    formatSize(bytes: number): string {
        if (bytes === 0) return '0 B';

        const k = 1024;
        const sizes = ['B', 'KB', 'MB'];
        const i = Math.floor(Math.log(bytes) / Math.log(k));

        return parseFloat((bytes / Math.pow(k, i)).toFixed(1)) + ' ' + sizes[i];
    }
}
```

## Image Preview

Grid-based image preview with thumbnails. Hover over images to reveal the remove button.

```typescript
import { Component } from '@angular/core';
import { FileUploadModule } from 'primeng/fileupload';
import { ButtonModule } from 'primeng/button';
import { Plus } from '@primeicons/angular/plus';
import { Upload } from '@primeicons/angular/upload';
import { Times } from '@primeicons/angular/times';

@Component({
    template: `
        <p-fileupload #fu name="demo[]" url="https://www.primefaces.org/cdn/api/upload.php" [multiple]="true" accept="image/*" maxFileSize="1000000" mode="advanced" [pt]="{ root: { class: 'border-0!' } }">
            <ng-template #header let-files let-chooseCallback="chooseCallback" let-uploadCallback="uploadCallback">
                <div class="flex items-center gap-2">
                    <button type="button" pButton severity="secondary" variant="outlined" (click)="chooseCallback()">
                        <svg data-p-icon="plus"></svg>
                        Add Images
                    </button>
                    @if (files?.length > 0) {
                        <button type="button" pButton (click)="uploadCallback()">
                            <svg data-p-icon="upload"></svg>
                            Upload All
                        </button>
                    }
                </div>
            </ng-template>
            <ng-template #content let-files let-removeFileCallback="removeFileCallback">
                @if (files?.length > 0) {
                    <div class="grid grid-cols-2 sm:grid-cols-3 md:grid-cols-4 gap-4">
                        @for (file of files; track $index; let i = $index) {
                            <div class="group relative rounded-lg overflow-hidden border border-surface-200 dark:border-surface-700">
                                <img [src]="file.objectURL" [alt]="file.name" class="w-full h-32 object-cover" />
                                <div class="absolute inset-0 bg-black/50 opacity-0 group-hover:opacity-100 transition-opacity flex items-center justify-center">
                                    <button type="button" class="text-white bg-red-500 rounded-full p-2 hover:bg-red-600 cursor-pointer border-0" (click)="removeFileCallback($event, i)">
                                        <svg data-p-icon="times"></svg>
                                    </button>
                                </div>
                                <div class="p-2">
                                    <div class="text-xs font-medium truncate">{{ file.name }}</div>
                                    <div class="text-xs text-muted-color">{{ formatSize(file.size) }}</div>
                                </div>
                            </div>
                        }
                    </div>
                }
            </ng-template>
            <ng-template #empty>
                <div class="border-2 border-dashed border-surface-200 dark:border-surface-700 rounded-xl p-12 text-center">
                    <p class="text-muted-color m-0!">No images selected. Click "Add Images" to get started.</p>
                </div>
            </ng-template>
        </p-fileupload>
    `,
    standalone: true,
    imports: [FileUploadModule, ButtonModule, Plus, Upload, Times]
})
export class FileUploadImagePreviewDemo {
    formatSize(bytes: number): string {
        if (bytes === 0) return '0 B';

        const k = 1024;
        const sizes = ['B', 'KB', 'MB'];
        const i = Math.floor(Math.log(bytes) / Math.log(k));

        return parseFloat((bytes / Math.pow(k, i)).toFixed(1)) + ' ' + sizes[i];
    }
}
```

## InputGroup

FileUpload can be placed inside an InputGroup as an addon to combine it with other form controls.

```typescript
import { Component, signal } from '@angular/core';
import { FileUploadModule } from 'primeng/fileupload';
import { InputGroupModule } from 'primeng/inputgroup';
import { InputGroupAddonModule } from 'primeng/inputgroupaddon';
import { ButtonModule } from 'primeng/button';
import { InputTextModule } from 'primeng/inputtext';
import { Upload } from '@primeicons/angular/upload';
import { CloudUpload } from '@primeicons/angular/cloud-upload';
import { Tag } from '@primeicons/angular/tag';

@Component({
    template: `
        <div class="flex flex-col gap-4 max-w-md mx-auto">
            <p-inputgroup>
                <p-inputgroup-addon>Upload</p-inputgroup-addon>
                <p-inputgroup-addon>
                    <p-fileupload
                        mode="basic"
                        accept="image/*"
                        [maxFileSize]="1000000"
                        [chooseLabel]="label1()"
                        [chooseButtonProps]="{ severity: 'secondary', variant: 'text', styleClass: 'flex-1 justify-start' }"
                        (onSelect)="onFileSelect($event, label1)"
                    >
                        <ng-template #chooseicon><svg data-p-icon="upload" /></ng-template>
                        <ng-template #filelabel></ng-template>
                    </p-fileupload>
                </p-inputgroup-addon>
            </p-inputgroup>
            <p-inputgroup>
                <p-inputgroup-addon>
                    <p-fileupload
                        mode="basic"
                        accept="image/*"
                        [maxFileSize]="1000000"
                        [chooseLabel]="label2()"
                        [chooseButtonProps]="{ severity: 'secondary', variant: 'text', styleClass: 'flex-1 justify-start' }"
                        (onSelect)="onFileSelect($event, label2)"
                    >
                        <ng-template #chooseicon><svg data-p-icon="upload" /></ng-template>
                        <ng-template #filelabel></ng-template>
                    </p-fileupload>
                </p-inputgroup-addon>
                <p-inputgroup-addon>
                    <button type="button" pButton severity="secondary" variant="text" iconOnly>
                        <svg data-p-icon="cloud-upload" />
                    </button>
                </p-inputgroup-addon>
            </p-inputgroup>
            <p-inputgroup>
                <p-inputgroup-addon>
                    <svg data-p-icon="tag" />
                </p-inputgroup-addon>
                <input pInputText placeholder="Label" />
                <p-inputgroup-addon>
                    <p-fileupload mode="basic" accept="image/*" [maxFileSize]="1000000" [chooseLabel]="label3()" [chooseButtonProps]="{ severity: 'secondary', variant: 'text' }" (onSelect)="onFileSelect($event, label3)">
                        <ng-template #chooseicon><svg data-p-icon="upload" /></ng-template>
                        <ng-template #filelabel></ng-template>
                    </p-fileupload>
                </p-inputgroup-addon>
            </p-inputgroup>
        </div>
    `,
    standalone: true,
    imports: [FileUploadModule, InputGroupModule, InputGroupAddonModule, ButtonModule, InputTextModule, Upload, CloudUpload, Tag]
})
export class FileUploadInputGroupDemo {
    label1 = signal('Choose file');

    label2 = signal('Choose file');

    label3 = signal('Browse');

    onFileSelect(event: FileSelectEvent, label: ReturnType<typeof signal<string>>) {
        const files: File[] = event.currentFiles || [];

        if (files.length) {
            label.set(files.map((f) => f.name).join(', '));
        }
    }
}
```

## preview-doc

```typescript
import { Component } from '@angular/core';
import { FileUploadModule } from 'primeng/fileupload';
import { ButtonModule } from 'primeng/button';
import { CloudUpload } from '@primeicons/angular/cloud-upload';
import { File } from '@primeicons/angular/file';
import { Times } from '@primeicons/angular/times';

@Component({
    template: `
        <div class="max-w-sm mx-auto">
            <p-fileupload
                #fu
                name="demo[]"
                url="https://www.primefaces.org/cdn/api/upload.php"
                [multiple]="true"
                maxFileSize="5000000"
                mode="advanced"
                [pt]="{ root: { class: 'border-0!' }, header: { class: 'hidden!' }, content: { class: 'p-0!' } }"
            >
                <ng-template #content let-files let-uploadedFiles="uploadedFiles" let-removeFileCallback="removeFileCallback" let-removeUploadedFileCallback="removeUploadedFileCallback">
                    <div class="flex flex-col items-center gap-2 p-6 cursor-pointer border-2 border-dashed border-surface rounded-md" (click)="fu.choose()">
                        <svg data-p-icon="cloud-upload" [size]="32" class="text-muted-color"></svg>
                        <div class="text-sm font-medium">Drop files or click to browse</div>
                        <div class="text-xs text-muted-color">Up to 5 MB each</div>
                    </div>
                    @if (uploadedFiles?.length > 0 || files?.length > 0) {
                        <div class="flex flex-col gap-2 mt-3">
                            @for (file of uploadedFiles; track $index; let i = $index) {
                                <div class="flex items-center gap-3 px-3 py-2 rounded-md bg-surface-50 dark:bg-surface-800">
                                    <svg data-p-icon="file" class="text-primary shrink-0"></svg>
                                    <div class="flex-1 min-w-0">
                                        <div class="text-sm truncate">{{ file.name }}</div>
                                        <div class="flex items-center gap-1 text-xs">
                                            <span class="text-muted-color tabular-nums">{{ formatSize(file.size) }}</span>
                                            <span class="text-muted-color">·</span>
                                            <span class="text-emerald-500">Completed</span>
                                        </div>
                                    </div>
                                    <button type="button" pButton iconOnly variant="text" severity="secondary" size="small" rounded (click)="removeUploadedFileCallback(i)"><svg data-p-icon="times"></svg></button>
                                </div>
                            }
                            @for (file of files; track $index; let i = $index) {
                                <div class="flex items-center gap-3 px-3 py-2 rounded-md bg-surface-50 dark:bg-surface-800">
                                    <svg data-p-icon="file" class="text-primary shrink-0"></svg>
                                    <div class="flex-1 min-w-0">
                                        <div class="text-sm truncate">{{ file.name }}</div>
                                        <div class="flex items-center gap-1 text-xs">
                                            <span class="text-muted-color tabular-nums">{{ formatSize(file.size) }}</span>
                                            <span class="text-muted-color">·</span>
                                            <span class="text-primary">Pending</span>
                                        </div>
                                    </div>
                                    <button type="button" pButton iconOnly variant="text" severity="secondary" size="small" rounded (click)="removeFileCallback($event, i)"><svg data-p-icon="times"></svg></button>
                                </div>
                            }
                        </div>
                    }
                    @if (files?.length > 0) {
                        <button type="button" pButton class="w-full! mt-3" (click)="fu.upload()">Upload {{ files.length }} {{ files.length === 1 ? 'file' : 'files' }}</button>
                    }
                </ng-template>
            </p-fileupload>
        </div>
    `,
    standalone: true,
    imports: [FileUploadModule, ButtonModule, CloudUpload, File, Times]
})
export class FileUploadPreviewDemo {
    formatSize(bytes: number): string {
        if (bytes === 0) return '0 B';

        const k = 1024;
        const sizes = ['B', 'KB', 'MB'];
        const i = Math.floor(Math.log(bytes) / Math.log(k));

        return parseFloat((bytes / Math.pow(k, i)).toFixed(1)) + ' ' + sizes[i];
    }
}
```

## File Upload

FileUpload is an advanced uploader with dragdrop support, multi file uploads, auto uploading, progress tracking and validations.

### Props

| Name | Type | Default | Description |
|------|------|---------|-------------|
| dt | object \| undefined | undefined | Defines scoped design tokens of the component. |
| unstyled | boolean \| undefined | undefined | Indicates whether the component should be rendered without styles. |
| pt | PassThrough<I, FileUploadPassThroughOptions<I>> | undefined | Used to pass attributes to DOM elements inside the component. |
| ptOptions | PassThroughOptions \| undefined | undefined | Used to configure passthrough(pt) options of the component. |
| name | string \| undefined | - | Name of the request parameter to identify the files at backend. |
| url | string \| undefined | - | Remote url to upload the files. |
| method | "post" \| "put" | - | HTTP method to send the files to the url such as "post" and "put". |
| multiple | boolean | - | Used to select multiple files at once from file dialog. |
| accept | string \| undefined | - | Comma-separated list of pattern to restrict the allowed file types. Can be any combination of either the MIME types (such as "image/*") or the file extensions (such as ".jpg"). |
| disabled | boolean | - | Disables the upload functionality. |
| auto | boolean | - | When enabled, upload begins automatically after selection is completed. |
| withCredentials | boolean | - | Cross-site Access-Control requests should be made using credentials such as cookies, authorization headers or TLS client certificates. |
| maxFileSize | number \| undefined | - | Maximum file size allowed in bytes. |
| invalidFileSizeMessageSummary | string | - | Summary message of the invalid file size. |
| invalidFileSizeMessageDetail | string | - | Detail message of the invalid file size. |
| invalidFileTypeMessageSummary | string | - | Summary message of the invalid file type. |
| invalidFileTypeMessageDetail | string | - | Detail message of the invalid file type. |
| invalidFileLimitMessageDetail | string | - | Detail message of the invalid file type. |
| invalidFileLimitMessageSummary | string | - | Summary message of the invalid file type. |
| style | { [klass: string]: any } \| null \| undefined | - | Inline style of the element. |
| styleClass | string \| undefined | - | Class of the element. |
| previewWidth | number | - | Width of the image thumbnail in pixels. |
| chooseLabel | string \| undefined | - | Label of the choose button. Defaults to PrimeNG Locale configuration. |
| uploadLabel | string \| undefined | - | Label of the upload button. Defaults to PrimeNG Locale configuration. |
| cancelLabel | string \| undefined | - | Label of the cancel button. Defaults to PrimeNG Locale configuration. |
| chooseIcon | string \| undefined | - | Icon of the choose button. |
| uploadIcon | string \| undefined | - | Icon of the upload button. |
| cancelIcon | string \| undefined | - | Icon of the cancel button. |
| showUploadButton | boolean | - | Whether to show the upload button. |
| showCancelButton | boolean | - | Whether to show the cancel button. |
| mode | "advanced" \| "basic" | - | Defines the UI of the component. |
| headers | HttpHeaders \| undefined | - | HttpHeaders class represents the header configuration options for an HTTP request. |
| customUpload | boolean | - | Whether to use the default upload or a manual implementation defined in uploadHandler callback. Defaults to PrimeNG Locale configuration. |
| fileLimit | number \| undefined | - | Maximum number of files that can be uploaded. |
| uploadStyleClass | string \| undefined | - | Style class of the upload button. |
| cancelStyleClass | string \| undefined | - | Style class of the cancel button. |
| removeStyleClass | string \| undefined | - | Style class of the remove button. |
| chooseStyleClass | string \| undefined | - | Style class of the choose button. |
| chooseButtonProps | ButtonDirectiveOptions \| undefined | - | Configuration object forwarded to the choose button. |
| uploadButtonProps | ButtonDirectiveOptions | - | Configuration object forwarded to the upload button. |
| cancelButtonProps | ButtonDirectiveOptions | - | Configuration object forwarded to the cancel button. |
| filesInput | File[] | - | Files input. |

### Emits

| Name | Parameters | Description |
|------|------------|-------------|
| onBeforeUpload | event: FileBeforeUploadEvent | Callback to invoke before file upload is initialized. |
| onSend | event: FileSendEvent | An event indicating that the request was sent to the server. Useful when a request may be retried multiple times, to distinguish between retries on the final event stream. |
| onUpload | event: FileUploadEvent | Callback to invoke when file upload is complete. |
| onError | event: FileUploadErrorEvent | Callback to invoke if file upload fails. |
| onClear | value: void | Callback to invoke when files in queue are removed without uploading using clear all button. |
| onRemove | event: FileRemoveEvent | Callback to invoke when a file is removed without uploading using clear button of a file. |
| onSelect | event: FileSelectEvent | Callback to invoke when files are selected. |
| onProgress | event: FileProgressEvent | Callback to invoke when files are being uploaded. |
| uploadHandler | event: FileUploadHandlerEvent | Callback to invoke in custom upload mode to upload the files manually. |
| onImageError | event: Event | This event is triggered if an error occurs while loading an image file. |
| onRemoveUploadedFile | event: RemoveUploadedFileEvent | This event is triggered if an error occurs while loading an image file. |

### Templates

| Name | Type | Description |
|------|------|-------------|
| file | TemplateRef<FileUploadFileTemplateContext> \| undefined | Custom file template. |
| header | TemplateRef<FileUploadHeaderTemplateContext> \| undefined | Custom header template. |
| content | TemplateRef<FileUploadContentTemplateContext> \| undefined | Custom content template. |
| toolbar | TemplateRef<void> \| undefined | Custom toolbar template. |
| chooseicon | TemplateRef<void> \| undefined | Custom choose icon template. |
| filelabel | TemplateRef<FileUploadFileLabelTemplateContext> \| undefined | Custom file label template. |
| uploadicon | TemplateRef<void> \| undefined | Custom upload icon template. |
| cancelicon | TemplateRef<void> \| undefined | Custom cancel icon template. |
| empty | TemplateRef<void> \| undefined | Custom empty state template. |

### Methods

| Name | Parameters | Return Type | Description |
|------|------------|-------------|-------------|
| uploader |  | void | Uploads the selected files. |
| clear |  | void | Clears the files list. |
| remove | event: Event, index: number | void | Removes a single file. |
| removeUploadedFile | index: number | void | Removes uploaded file. |

## Pass Through Options

| Name | Type | Description |
|------|------|-------------|
| host | PassThroughOption<HTMLElement, I> | Used to pass attributes to the host's DOM element. |
| root | PassThroughOption<HTMLDivElement, I> | Used to pass attributes to the root's DOM element. |
| input | PassThroughOption<HTMLInputElement, I> | Used to pass attributes to the input's DOM element. |
| header | PassThroughOption<HTMLDivElement, I> | Used to pass attributes to the header's DOM element. |
| pcChooseButton | ButtonPassThrough | Used to pass attributes to the choose button component. |
| pcUploadButton | ButtonPassThrough | Used to pass attributes to the upload button component. |
| pcCancelButton | ButtonPassThrough | Used to pass attributes to the cancel button component. |
| content | PassThroughOption<HTMLDivElement, I> | Used to pass attributes to the content's DOM element. |
| pcProgressBar | ProgressBarPassThrough | Used to pass attributes to the progress bar component. |
| pcMessage | MessagePassThrough | Used to pass attributes to the message component. |
| fileList | PassThroughOption<HTMLDivElement, I> | Used to pass attributes to the file list's DOM element. |
| file | PassThroughOption<HTMLDivElement, I> | Used to pass attributes to the file's DOM element. |
| fileThumbnail | PassThroughOption<HTMLImageElement, I> | Used to pass attributes to the file thumbnail's DOM element. |
| fileInfo | PassThroughOption<HTMLDivElement, I> | Used to pass attributes to the file info's DOM element. |
| fileName | PassThroughOption<HTMLDivElement, I> | Used to pass attributes to the file name's DOM element. |
| fileSize | PassThroughOption<HTMLSpanElement, I> | Used to pass attributes to the file size's DOM element. |
| pcFileBadge | BadgePassThrough | Used to pass attributes to the file badge component. |
| fileActions | PassThroughOption<HTMLDivElement, I> | Used to pass attributes to the file actions's DOM element. |
| pcFileRemoveButton | ButtonPassThrough | Used to pass attributes to the file remove button component. |
| basicContent | PassThroughOption<HTMLDivElement, I> | Used to pass attributes to the basic content's DOM element. |
| empty | PassThroughOption<HTMLElement, I> | Used to pass attributes to the empty's DOM element. |

## Theming

### CSS Classes

| Class | Description |
|-------|-------------|
| p-fileupload | Class name of the root element |
| p-fileupload-header | Class name of the header element |
| p-fileupload-choose-button | Class name of the choose button element |
| p-fileupload-upload-button | Class name of the upload button element |
| p-fileupload-cancel-button | Class name of the cancel button element |
| p-fileupload-content | Class name of the content element |
| p-fileupload-file-list | Class name of the file list element |
| p-fileupload-file | Class name of the file element |
| p-fileupload-file-thumbnail | Class name of the file thumbnail element |
| p-fileupload-file-info | Class name of the file info element |
| p-fileupload-file-name | Class name of the file name element |
| p-fileupload-file-size | Class name of the file size element |
| p-fileupload-file-badge | Class name of the file badge element |
| p-fileupload-file-actions | Class name of the file actions element |
| p-fileupload-file-remove-button | Class name of the file remove button element |
| p-fileupload-basic-content | Class name of the content in basic mode |

### Design Tokens

| Token | CSS Variable | Description |
|-------|--------------|-------------|
| fileupload.background | --p-fileupload-background | Background of root |
| fileupload.border.color | --p-fileupload-border-color | Border color of root |
| fileupload.color | --p-fileupload-color | Color of root |
| fileupload.border.radius | --p-fileupload-border-radius | Border radius of root |
| fileupload.transition.duration | --p-fileupload-transition-duration | Transition duration of root |
| fileupload.header.background | --p-fileupload-header-background | Background of header |
| fileupload.header.color | --p-fileupload-header-color | Color of header |
| fileupload.header.padding | --p-fileupload-header-padding | Padding of header |
| fileupload.header.border.color | --p-fileupload-header-border-color | Border color of header |
| fileupload.header.border.width | --p-fileupload-header-border-width | Border width of header |
| fileupload.header.border.radius | --p-fileupload-header-border-radius | Border radius of header |
| fileupload.header.gap | --p-fileupload-header-gap | Gap of header |
| fileupload.content.highlight.border.color | --p-fileupload-content-highlight-border-color | Highlight border color of content |
| fileupload.content.padding | --p-fileupload-content-padding | Padding of content |
| fileupload.content.gap | --p-fileupload-content-gap | Gap of content |
| fileupload.file.padding | --p-fileupload-file-padding | Padding of file |
| fileupload.file.gap | --p-fileupload-file-gap | Gap of file |
| fileupload.file.border.color | --p-fileupload-file-border-color | Border color of file |
| fileupload.file.info.gap | --p-fileupload-file-info-gap | Info gap of file |
| fileupload.file.name.color | --p-fileupload-file-name-color | Color of file name |
| fileupload.file.name.font.weight | --p-fileupload-file-name-font-weight | Font weight of file name |
| fileupload.file.name.font.size | --p-fileupload-file-name-font-size | Font size of file name |
| fileupload.file.size.color | --p-fileupload-file-size-color | Color of file size |
| fileupload.file.size.font.weight | --p-fileupload-file-size-font-weight | Font weight of file size |
| fileupload.file.size.font.size | --p-fileupload-file-size-font-size | Font size of file size |
| fileupload.file.list.gap | --p-fileupload-file-list-gap | Gap of file list |
| fileupload.progressbar.height | --p-fileupload-progressbar-height | Height of progressbar |
| fileupload.basic.gap | --p-fileupload-basic-gap | Gap of basic |
