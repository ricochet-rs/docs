{{- if . }}
{{- range . }}
### Target `{{ .Target }}`

{{- if (eq (len .Vulnerabilities) 0) }}
#### ✅ No Vulnerabilities found
{{- else }}
#### ⚠️ Vulnerabilities ({{ len .Vulnerabilities }})

| Package | ID | Severity | Installed Version | Fixed Version | Links |
|---------|----|---------:|------------------:|--------------|-------|
{{- range .Vulnerabilities }}
| `{{ .PkgName }}` | {{ .VulnerabilityID }} | **{{ .Severity }}** | {{ .InstalledVersion }} | {{ .FixedVersion }} | {{- range .References }}[🔗]({{ . }}) {{ end }} |
{{- end }}
{{- end }}

{{- if (eq (len .Misconfigurations ) 0) }}
#### ✅ No Misconfigurations found
{{- else }}
#### ⚠️ Misconfigurations ({{ len .Misconfigurations }})

| Type | ID | Check | Severity | Message |
|------|-------|-------|----------|---------|
{{- range .Misconfigurations }}
| {{ .Type }} | {{ .ID }} | {{ .Title }} | **{{ .Severity }}** | {{ .Message }}<br>[📖 Details]({{ .PrimaryURL }}) |
{{- end }}
{{- end }}

{{- end }}
{{- else }}
### ℹ️ Trivy Returned Empty Report
{{- end }}
