// Generated by idl2rs.

/// Wrapper for `ICoreWebView2`.
#[derive(Clone)]
pub struct WebView {
    inner: ComRc<dyn ICoreWebView2>,
}
impl From<ComRc<dyn ICoreWebView2>> for WebView {
    fn from(inner: ComRc<dyn ICoreWebView2>) -> Self {
        Self { inner }
    }
}
impl fmt::Debug for WebView {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("WebView").finish()
    }
}
impl WebView {
    pub fn into_inner(self) -> ComRc<dyn ICoreWebView2> {
        self.inner
    }
    pub fn as_inner(&self) -> &ComRc<dyn ICoreWebView2> {
        &self.inner
    }
}

/// Wrapper for `ICoreWebView2_2`.
#[derive(Clone)]
pub struct WebView_2 {
    inner: ComRc<dyn ICoreWebView2_2>,
}
impl From<ComRc<dyn ICoreWebView2_2>> for WebView_2 {
    fn from(inner: ComRc<dyn ICoreWebView2_2>) -> Self {
        Self { inner }
    }
}
impl fmt::Debug for WebView_2 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("WebView_2").finish()
    }
}
impl WebView_2 {
    pub fn into_inner(self) -> ComRc<dyn ICoreWebView2_2> {
        self.inner
    }
    pub fn as_inner(&self) -> &ComRc<dyn ICoreWebView2_2> {
        &self.inner
    }
}

/// Wrapper for `ICoreWebView2_3`.
#[derive(Clone)]
pub struct WebView_3 {
    inner: ComRc<dyn ICoreWebView2_3>,
}
impl From<ComRc<dyn ICoreWebView2_3>> for WebView_3 {
    fn from(inner: ComRc<dyn ICoreWebView2_3>) -> Self {
        Self { inner }
    }
}
impl fmt::Debug for WebView_3 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("WebView_3").finish()
    }
}
impl WebView_3 {
    pub fn into_inner(self) -> ComRc<dyn ICoreWebView2_3> {
        self.inner
    }
    pub fn as_inner(&self) -> &ComRc<dyn ICoreWebView2_3> {
        &self.inner
    }
}

/// Wrapper for `ICoreWebView2Controller`.
#[derive(Clone)]
pub struct Controller {
    inner: ComRc<dyn ICoreWebView2Controller>,
}
impl From<ComRc<dyn ICoreWebView2Controller>> for Controller {
    fn from(inner: ComRc<dyn ICoreWebView2Controller>) -> Self {
        Self { inner }
    }
}
impl fmt::Debug for Controller {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Controller").finish()
    }
}
impl Controller {
    pub fn into_inner(self) -> ComRc<dyn ICoreWebView2Controller> {
        self.inner
    }
    pub fn as_inner(&self) -> &ComRc<dyn ICoreWebView2Controller> {
        &self.inner
    }
}

/// Wrapper for `ICoreWebView2Controller2`.
#[derive(Clone)]
pub struct Controller2 {
    inner: ComRc<dyn ICoreWebView2Controller2>,
}
impl From<ComRc<dyn ICoreWebView2Controller2>> for Controller2 {
    fn from(inner: ComRc<dyn ICoreWebView2Controller2>) -> Self {
        Self { inner }
    }
}
impl fmt::Debug for Controller2 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Controller2").finish()
    }
}
impl Controller2 {
    pub fn into_inner(self) -> ComRc<dyn ICoreWebView2Controller2> {
        self.inner
    }
    pub fn as_inner(&self) -> &ComRc<dyn ICoreWebView2Controller2> {
        &self.inner
    }
}

/// Wrapper for `ICoreWebView2Controller3`.
#[derive(Clone)]
pub struct Controller3 {
    inner: ComRc<dyn ICoreWebView2Controller3>,
}
impl From<ComRc<dyn ICoreWebView2Controller3>> for Controller3 {
    fn from(inner: ComRc<dyn ICoreWebView2Controller3>) -> Self {
        Self { inner }
    }
}
impl fmt::Debug for Controller3 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Controller3").finish()
    }
}
impl Controller3 {
    pub fn into_inner(self) -> ComRc<dyn ICoreWebView2Controller3> {
        self.inner
    }
    pub fn as_inner(&self) -> &ComRc<dyn ICoreWebView2Controller3> {
        &self.inner
    }
}

/// Wrapper for `ICoreWebView2CompositionController`.
#[derive(Clone)]
pub struct CompositionController {
    inner: ComRc<dyn ICoreWebView2CompositionController>,
}
impl From<ComRc<dyn ICoreWebView2CompositionController>> for CompositionController {
    fn from(inner: ComRc<dyn ICoreWebView2CompositionController>) -> Self {
        Self { inner }
    }
}
impl fmt::Debug for CompositionController {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("CompositionController").finish()
    }
}
impl CompositionController {
    pub fn into_inner(self) -> ComRc<dyn ICoreWebView2CompositionController> {
        self.inner
    }
    pub fn as_inner(&self) -> &ComRc<dyn ICoreWebView2CompositionController> {
        &self.inner
    }
}

/// Wrapper for `ICoreWebView2CompositionController2`.
#[derive(Clone)]
pub struct CompositionController2 {
    inner: ComRc<dyn ICoreWebView2CompositionController2>,
}
impl From<ComRc<dyn ICoreWebView2CompositionController2>> for CompositionController2 {
    fn from(inner: ComRc<dyn ICoreWebView2CompositionController2>) -> Self {
        Self { inner }
    }
}
impl fmt::Debug for CompositionController2 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("CompositionController2").finish()
    }
}
impl CompositionController2 {
    pub fn into_inner(self) -> ComRc<dyn ICoreWebView2CompositionController2> {
        self.inner
    }
    pub fn as_inner(&self) -> &ComRc<dyn ICoreWebView2CompositionController2> {
        &self.inner
    }
}

/// Wrapper for `ICoreWebView2Deferral`.
#[derive(Clone)]
pub struct Deferral {
    inner: ComRc<dyn ICoreWebView2Deferral>,
}
impl From<ComRc<dyn ICoreWebView2Deferral>> for Deferral {
    fn from(inner: ComRc<dyn ICoreWebView2Deferral>) -> Self {
        Self { inner }
    }
}
impl fmt::Debug for Deferral {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Deferral").finish()
    }
}
impl Deferral {
    pub fn into_inner(self) -> ComRc<dyn ICoreWebView2Deferral> {
        self.inner
    }
    pub fn as_inner(&self) -> &ComRc<dyn ICoreWebView2Deferral> {
        &self.inner
    }
}

/// Wrapper for `ICoreWebView2Settings`.
#[derive(Clone)]
pub struct Settings {
    inner: ComRc<dyn ICoreWebView2Settings>,
}
impl From<ComRc<dyn ICoreWebView2Settings>> for Settings {
    fn from(inner: ComRc<dyn ICoreWebView2Settings>) -> Self {
        Self { inner }
    }
}
impl fmt::Debug for Settings {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Settings").finish()
    }
}
impl Settings {
    pub fn into_inner(self) -> ComRc<dyn ICoreWebView2Settings> {
        self.inner
    }
    pub fn as_inner(&self) -> &ComRc<dyn ICoreWebView2Settings> {
        &self.inner
    }
}

/// Wrapper for `ICoreWebView2ProcessFailedEventArgs`.
#[derive(Clone)]
pub struct ProcessFailedEventArgs {
    inner: ComRc<dyn ICoreWebView2ProcessFailedEventArgs>,
}
impl From<ComRc<dyn ICoreWebView2ProcessFailedEventArgs>> for ProcessFailedEventArgs {
    fn from(inner: ComRc<dyn ICoreWebView2ProcessFailedEventArgs>) -> Self {
        Self { inner }
    }
}
impl fmt::Debug for ProcessFailedEventArgs {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("ProcessFailedEventArgs").finish()
    }
}
impl ProcessFailedEventArgs {
    pub fn into_inner(self) -> ComRc<dyn ICoreWebView2ProcessFailedEventArgs> {
        self.inner
    }
    pub fn as_inner(&self) -> &ComRc<dyn ICoreWebView2ProcessFailedEventArgs> {
        &self.inner
    }
}

/// Wrapper for `ICoreWebView2HttpHeadersCollectionIterator`.
#[derive(Clone)]
pub struct HttpHeadersCollectionIterator {
    inner: ComRc<dyn ICoreWebView2HttpHeadersCollectionIterator>,
}
impl From<ComRc<dyn ICoreWebView2HttpHeadersCollectionIterator>> for HttpHeadersCollectionIterator {
    fn from(inner: ComRc<dyn ICoreWebView2HttpHeadersCollectionIterator>) -> Self {
        Self { inner }
    }
}
impl fmt::Debug for HttpHeadersCollectionIterator {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("HttpHeadersCollectionIterator").finish()
    }
}
impl HttpHeadersCollectionIterator {
    pub fn into_inner(self) -> ComRc<dyn ICoreWebView2HttpHeadersCollectionIterator> {
        self.inner
    }
    pub fn as_inner(&self) -> &ComRc<dyn ICoreWebView2HttpHeadersCollectionIterator> {
        &self.inner
    }
}

/// Wrapper for `ICoreWebView2HttpRequestHeaders`.
#[derive(Clone)]
pub struct HttpRequestHeaders {
    inner: ComRc<dyn ICoreWebView2HttpRequestHeaders>,
}
impl From<ComRc<dyn ICoreWebView2HttpRequestHeaders>> for HttpRequestHeaders {
    fn from(inner: ComRc<dyn ICoreWebView2HttpRequestHeaders>) -> Self {
        Self { inner }
    }
}
impl fmt::Debug for HttpRequestHeaders {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("HttpRequestHeaders").finish()
    }
}
impl HttpRequestHeaders {
    pub fn into_inner(self) -> ComRc<dyn ICoreWebView2HttpRequestHeaders> {
        self.inner
    }
    pub fn as_inner(&self) -> &ComRc<dyn ICoreWebView2HttpRequestHeaders> {
        &self.inner
    }
}

/// Wrapper for `ICoreWebView2HttpResponseHeaders`.
#[derive(Clone)]
pub struct HttpResponseHeaders {
    inner: ComRc<dyn ICoreWebView2HttpResponseHeaders>,
}
impl From<ComRc<dyn ICoreWebView2HttpResponseHeaders>> for HttpResponseHeaders {
    fn from(inner: ComRc<dyn ICoreWebView2HttpResponseHeaders>) -> Self {
        Self { inner }
    }
}
impl fmt::Debug for HttpResponseHeaders {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("HttpResponseHeaders").finish()
    }
}
impl HttpResponseHeaders {
    pub fn into_inner(self) -> ComRc<dyn ICoreWebView2HttpResponseHeaders> {
        self.inner
    }
    pub fn as_inner(&self) -> &ComRc<dyn ICoreWebView2HttpResponseHeaders> {
        &self.inner
    }
}

/// Wrapper for `ICoreWebView2WebResourceRequest`.
#[derive(Clone)]
pub struct WebResourceRequest {
    inner: ComRc<dyn ICoreWebView2WebResourceRequest>,
}
impl From<ComRc<dyn ICoreWebView2WebResourceRequest>> for WebResourceRequest {
    fn from(inner: ComRc<dyn ICoreWebView2WebResourceRequest>) -> Self {
        Self { inner }
    }
}
impl fmt::Debug for WebResourceRequest {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("WebResourceRequest").finish()
    }
}
impl WebResourceRequest {
    pub fn into_inner(self) -> ComRc<dyn ICoreWebView2WebResourceRequest> {
        self.inner
    }
    pub fn as_inner(&self) -> &ComRc<dyn ICoreWebView2WebResourceRequest> {
        &self.inner
    }
}

/// Wrapper for `ICoreWebView2WebResourceResponse`.
#[derive(Clone)]
pub struct WebResourceResponse {
    inner: ComRc<dyn ICoreWebView2WebResourceResponse>,
}
impl From<ComRc<dyn ICoreWebView2WebResourceResponse>> for WebResourceResponse {
    fn from(inner: ComRc<dyn ICoreWebView2WebResourceResponse>) -> Self {
        Self { inner }
    }
}
impl fmt::Debug for WebResourceResponse {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("WebResourceResponse").finish()
    }
}
impl WebResourceResponse {
    pub fn into_inner(self) -> ComRc<dyn ICoreWebView2WebResourceResponse> {
        self.inner
    }
    pub fn as_inner(&self) -> &ComRc<dyn ICoreWebView2WebResourceResponse> {
        &self.inner
    }
}

/// Wrapper for `ICoreWebView2NavigationStartingEventArgs`.
#[derive(Clone)]
pub struct NavigationStartingEventArgs {
    inner: ComRc<dyn ICoreWebView2NavigationStartingEventArgs>,
}
impl From<ComRc<dyn ICoreWebView2NavigationStartingEventArgs>> for NavigationStartingEventArgs {
    fn from(inner: ComRc<dyn ICoreWebView2NavigationStartingEventArgs>) -> Self {
        Self { inner }
    }
}
impl fmt::Debug for NavigationStartingEventArgs {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("NavigationStartingEventArgs").finish()
    }
}
impl NavigationStartingEventArgs {
    pub fn into_inner(self) -> ComRc<dyn ICoreWebView2NavigationStartingEventArgs> {
        self.inner
    }
    pub fn as_inner(&self) -> &ComRc<dyn ICoreWebView2NavigationStartingEventArgs> {
        &self.inner
    }
}

/// Wrapper for `ICoreWebView2ContentLoadingEventArgs`.
#[derive(Clone)]
pub struct ContentLoadingEventArgs {
    inner: ComRc<dyn ICoreWebView2ContentLoadingEventArgs>,
}
impl From<ComRc<dyn ICoreWebView2ContentLoadingEventArgs>> for ContentLoadingEventArgs {
    fn from(inner: ComRc<dyn ICoreWebView2ContentLoadingEventArgs>) -> Self {
        Self { inner }
    }
}
impl fmt::Debug for ContentLoadingEventArgs {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("ContentLoadingEventArgs").finish()
    }
}
impl ContentLoadingEventArgs {
    pub fn into_inner(self) -> ComRc<dyn ICoreWebView2ContentLoadingEventArgs> {
        self.inner
    }
    pub fn as_inner(&self) -> &ComRc<dyn ICoreWebView2ContentLoadingEventArgs> {
        &self.inner
    }
}

/// Wrapper for `ICoreWebView2SourceChangedEventArgs`.
#[derive(Clone)]
pub struct SourceChangedEventArgs {
    inner: ComRc<dyn ICoreWebView2SourceChangedEventArgs>,
}
impl From<ComRc<dyn ICoreWebView2SourceChangedEventArgs>> for SourceChangedEventArgs {
    fn from(inner: ComRc<dyn ICoreWebView2SourceChangedEventArgs>) -> Self {
        Self { inner }
    }
}
impl fmt::Debug for SourceChangedEventArgs {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("SourceChangedEventArgs").finish()
    }
}
impl SourceChangedEventArgs {
    pub fn into_inner(self) -> ComRc<dyn ICoreWebView2SourceChangedEventArgs> {
        self.inner
    }
    pub fn as_inner(&self) -> &ComRc<dyn ICoreWebView2SourceChangedEventArgs> {
        &self.inner
    }
}

/// Wrapper for `ICoreWebView2ScriptDialogOpeningEventArgs`.
#[derive(Clone)]
pub struct ScriptDialogOpeningEventArgs {
    inner: ComRc<dyn ICoreWebView2ScriptDialogOpeningEventArgs>,
}
impl From<ComRc<dyn ICoreWebView2ScriptDialogOpeningEventArgs>> for ScriptDialogOpeningEventArgs {
    fn from(inner: ComRc<dyn ICoreWebView2ScriptDialogOpeningEventArgs>) -> Self {
        Self { inner }
    }
}
impl fmt::Debug for ScriptDialogOpeningEventArgs {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("ScriptDialogOpeningEventArgs").finish()
    }
}
impl ScriptDialogOpeningEventArgs {
    pub fn into_inner(self) -> ComRc<dyn ICoreWebView2ScriptDialogOpeningEventArgs> {
        self.inner
    }
    pub fn as_inner(&self) -> &ComRc<dyn ICoreWebView2ScriptDialogOpeningEventArgs> {
        &self.inner
    }
}

/// Wrapper for `ICoreWebView2NavigationCompletedEventArgs`.
#[derive(Clone)]
pub struct NavigationCompletedEventArgs {
    inner: ComRc<dyn ICoreWebView2NavigationCompletedEventArgs>,
}
impl From<ComRc<dyn ICoreWebView2NavigationCompletedEventArgs>> for NavigationCompletedEventArgs {
    fn from(inner: ComRc<dyn ICoreWebView2NavigationCompletedEventArgs>) -> Self {
        Self { inner }
    }
}
impl fmt::Debug for NavigationCompletedEventArgs {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("NavigationCompletedEventArgs").finish()
    }
}
impl NavigationCompletedEventArgs {
    pub fn into_inner(self) -> ComRc<dyn ICoreWebView2NavigationCompletedEventArgs> {
        self.inner
    }
    pub fn as_inner(&self) -> &ComRc<dyn ICoreWebView2NavigationCompletedEventArgs> {
        &self.inner
    }
}

/// Wrapper for `ICoreWebView2PermissionRequestedEventArgs`.
#[derive(Clone)]
pub struct PermissionRequestedEventArgs {
    inner: ComRc<dyn ICoreWebView2PermissionRequestedEventArgs>,
}
impl From<ComRc<dyn ICoreWebView2PermissionRequestedEventArgs>> for PermissionRequestedEventArgs {
    fn from(inner: ComRc<dyn ICoreWebView2PermissionRequestedEventArgs>) -> Self {
        Self { inner }
    }
}
impl fmt::Debug for PermissionRequestedEventArgs {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("PermissionRequestedEventArgs").finish()
    }
}
impl PermissionRequestedEventArgs {
    pub fn into_inner(self) -> ComRc<dyn ICoreWebView2PermissionRequestedEventArgs> {
        self.inner
    }
    pub fn as_inner(&self) -> &ComRc<dyn ICoreWebView2PermissionRequestedEventArgs> {
        &self.inner
    }
}

/// Wrapper for `ICoreWebView2WebResourceRequestedEventArgs`.
#[derive(Clone)]
pub struct WebResourceRequestedEventArgs {
    inner: ComRc<dyn ICoreWebView2WebResourceRequestedEventArgs>,
}
impl From<ComRc<dyn ICoreWebView2WebResourceRequestedEventArgs>> for WebResourceRequestedEventArgs {
    fn from(inner: ComRc<dyn ICoreWebView2WebResourceRequestedEventArgs>) -> Self {
        Self { inner }
    }
}
impl fmt::Debug for WebResourceRequestedEventArgs {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("WebResourceRequestedEventArgs").finish()
    }
}
impl WebResourceRequestedEventArgs {
    pub fn into_inner(self) -> ComRc<dyn ICoreWebView2WebResourceRequestedEventArgs> {
        self.inner
    }
    pub fn as_inner(&self) -> &ComRc<dyn ICoreWebView2WebResourceRequestedEventArgs> {
        &self.inner
    }
}

/// Wrapper for `ICoreWebView2MoveFocusRequestedEventArgs`.
#[derive(Clone)]
pub struct MoveFocusRequestedEventArgs {
    inner: ComRc<dyn ICoreWebView2MoveFocusRequestedEventArgs>,
}
impl From<ComRc<dyn ICoreWebView2MoveFocusRequestedEventArgs>> for MoveFocusRequestedEventArgs {
    fn from(inner: ComRc<dyn ICoreWebView2MoveFocusRequestedEventArgs>) -> Self {
        Self { inner }
    }
}
impl fmt::Debug for MoveFocusRequestedEventArgs {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("MoveFocusRequestedEventArgs").finish()
    }
}
impl MoveFocusRequestedEventArgs {
    pub fn into_inner(self) -> ComRc<dyn ICoreWebView2MoveFocusRequestedEventArgs> {
        self.inner
    }
    pub fn as_inner(&self) -> &ComRc<dyn ICoreWebView2MoveFocusRequestedEventArgs> {
        &self.inner
    }
}

/// Wrapper for `ICoreWebView2WebMessageReceivedEventArgs`.
#[derive(Clone)]
pub struct WebMessageReceivedEventArgs {
    inner: ComRc<dyn ICoreWebView2WebMessageReceivedEventArgs>,
}
impl From<ComRc<dyn ICoreWebView2WebMessageReceivedEventArgs>> for WebMessageReceivedEventArgs {
    fn from(inner: ComRc<dyn ICoreWebView2WebMessageReceivedEventArgs>) -> Self {
        Self { inner }
    }
}
impl fmt::Debug for WebMessageReceivedEventArgs {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("WebMessageReceivedEventArgs").finish()
    }
}
impl WebMessageReceivedEventArgs {
    pub fn into_inner(self) -> ComRc<dyn ICoreWebView2WebMessageReceivedEventArgs> {
        self.inner
    }
    pub fn as_inner(&self) -> &ComRc<dyn ICoreWebView2WebMessageReceivedEventArgs> {
        &self.inner
    }
}

/// Wrapper for `ICoreWebView2DevToolsProtocolEventReceivedEventArgs`.
#[derive(Clone)]
pub struct DevToolsProtocolEventReceivedEventArgs {
    inner: ComRc<dyn ICoreWebView2DevToolsProtocolEventReceivedEventArgs>,
}
impl From<ComRc<dyn ICoreWebView2DevToolsProtocolEventReceivedEventArgs>>
    for DevToolsProtocolEventReceivedEventArgs
{
    fn from(inner: ComRc<dyn ICoreWebView2DevToolsProtocolEventReceivedEventArgs>) -> Self {
        Self { inner }
    }
}
impl fmt::Debug for DevToolsProtocolEventReceivedEventArgs {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("DevToolsProtocolEventReceivedEventArgs")
            .finish()
    }
}
impl DevToolsProtocolEventReceivedEventArgs {
    pub fn into_inner(self) -> ComRc<dyn ICoreWebView2DevToolsProtocolEventReceivedEventArgs> {
        self.inner
    }
    pub fn as_inner(&self) -> &ComRc<dyn ICoreWebView2DevToolsProtocolEventReceivedEventArgs> {
        &self.inner
    }
}

/// Wrapper for `ICoreWebView2NewWindowRequestedEventArgs`.
#[derive(Clone)]
pub struct NewWindowRequestedEventArgs {
    inner: ComRc<dyn ICoreWebView2NewWindowRequestedEventArgs>,
}
impl From<ComRc<dyn ICoreWebView2NewWindowRequestedEventArgs>> for NewWindowRequestedEventArgs {
    fn from(inner: ComRc<dyn ICoreWebView2NewWindowRequestedEventArgs>) -> Self {
        Self { inner }
    }
}
impl fmt::Debug for NewWindowRequestedEventArgs {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("NewWindowRequestedEventArgs").finish()
    }
}
impl NewWindowRequestedEventArgs {
    pub fn into_inner(self) -> ComRc<dyn ICoreWebView2NewWindowRequestedEventArgs> {
        self.inner
    }
    pub fn as_inner(&self) -> &ComRc<dyn ICoreWebView2NewWindowRequestedEventArgs> {
        &self.inner
    }
}

/// Wrapper for `ICoreWebView2WindowFeatures`.
#[derive(Clone)]
pub struct WindowFeatures {
    inner: ComRc<dyn ICoreWebView2WindowFeatures>,
}
impl From<ComRc<dyn ICoreWebView2WindowFeatures>> for WindowFeatures {
    fn from(inner: ComRc<dyn ICoreWebView2WindowFeatures>) -> Self {
        Self { inner }
    }
}
impl fmt::Debug for WindowFeatures {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("WindowFeatures").finish()
    }
}
impl WindowFeatures {
    pub fn into_inner(self) -> ComRc<dyn ICoreWebView2WindowFeatures> {
        self.inner
    }
    pub fn as_inner(&self) -> &ComRc<dyn ICoreWebView2WindowFeatures> {
        &self.inner
    }
}

/// Wrapper for `ICoreWebView2AcceleratorKeyPressedEventArgs`.
#[derive(Clone)]
pub struct AcceleratorKeyPressedEventArgs {
    inner: ComRc<dyn ICoreWebView2AcceleratorKeyPressedEventArgs>,
}
impl From<ComRc<dyn ICoreWebView2AcceleratorKeyPressedEventArgs>>
    for AcceleratorKeyPressedEventArgs
{
    fn from(inner: ComRc<dyn ICoreWebView2AcceleratorKeyPressedEventArgs>) -> Self {
        Self { inner }
    }
}
impl fmt::Debug for AcceleratorKeyPressedEventArgs {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("AcceleratorKeyPressedEventArgs").finish()
    }
}
impl AcceleratorKeyPressedEventArgs {
    pub fn into_inner(self) -> ComRc<dyn ICoreWebView2AcceleratorKeyPressedEventArgs> {
        self.inner
    }
    pub fn as_inner(&self) -> &ComRc<dyn ICoreWebView2AcceleratorKeyPressedEventArgs> {
        &self.inner
    }
}

/// Wrapper for `ICoreWebView2WebResourceResponseReceivedEventArgs`.
#[derive(Clone)]
pub struct WebResourceResponseReceivedEventArgs {
    inner: ComRc<dyn ICoreWebView2WebResourceResponseReceivedEventArgs>,
}
impl From<ComRc<dyn ICoreWebView2WebResourceResponseReceivedEventArgs>>
    for WebResourceResponseReceivedEventArgs
{
    fn from(inner: ComRc<dyn ICoreWebView2WebResourceResponseReceivedEventArgs>) -> Self {
        Self { inner }
    }
}
impl fmt::Debug for WebResourceResponseReceivedEventArgs {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("WebResourceResponseReceivedEventArgs")
            .finish()
    }
}
impl WebResourceResponseReceivedEventArgs {
    pub fn into_inner(self) -> ComRc<dyn ICoreWebView2WebResourceResponseReceivedEventArgs> {
        self.inner
    }
    pub fn as_inner(&self) -> &ComRc<dyn ICoreWebView2WebResourceResponseReceivedEventArgs> {
        &self.inner
    }
}

/// Wrapper for `ICoreWebView2WebResourceResponseView`.
#[derive(Clone)]
pub struct WebResourceResponseView {
    inner: ComRc<dyn ICoreWebView2WebResourceResponseView>,
}
impl From<ComRc<dyn ICoreWebView2WebResourceResponseView>> for WebResourceResponseView {
    fn from(inner: ComRc<dyn ICoreWebView2WebResourceResponseView>) -> Self {
        Self { inner }
    }
}
impl fmt::Debug for WebResourceResponseView {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("WebResourceResponseView").finish()
    }
}
impl WebResourceResponseView {
    pub fn into_inner(self) -> ComRc<dyn ICoreWebView2WebResourceResponseView> {
        self.inner
    }
    pub fn as_inner(&self) -> &ComRc<dyn ICoreWebView2WebResourceResponseView> {
        &self.inner
    }
}

/// Wrapper for `ICoreWebView2DOMContentLoadedEventArgs`.
#[derive(Clone)]
pub struct DOMContentLoadedEventArgs {
    inner: ComRc<dyn ICoreWebView2DOMContentLoadedEventArgs>,
}
impl From<ComRc<dyn ICoreWebView2DOMContentLoadedEventArgs>> for DOMContentLoadedEventArgs {
    fn from(inner: ComRc<dyn ICoreWebView2DOMContentLoadedEventArgs>) -> Self {
        Self { inner }
    }
}
impl fmt::Debug for DOMContentLoadedEventArgs {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("DOMContentLoadedEventArgs").finish()
    }
}
impl DOMContentLoadedEventArgs {
    pub fn into_inner(self) -> ComRc<dyn ICoreWebView2DOMContentLoadedEventArgs> {
        self.inner
    }
    pub fn as_inner(&self) -> &ComRc<dyn ICoreWebView2DOMContentLoadedEventArgs> {
        &self.inner
    }
}

/// Wrapper for `ICoreWebView2Cookie`.
#[derive(Clone)]
pub struct Cookie {
    inner: ComRc<dyn ICoreWebView2Cookie>,
}
impl From<ComRc<dyn ICoreWebView2Cookie>> for Cookie {
    fn from(inner: ComRc<dyn ICoreWebView2Cookie>) -> Self {
        Self { inner }
    }
}
impl fmt::Debug for Cookie {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Cookie").finish()
    }
}
impl Cookie {
    pub fn into_inner(self) -> ComRc<dyn ICoreWebView2Cookie> {
        self.inner
    }
    pub fn as_inner(&self) -> &ComRc<dyn ICoreWebView2Cookie> {
        &self.inner
    }
}

/// Wrapper for `ICoreWebView2CookieManager`.
#[derive(Clone)]
pub struct CookieManager {
    inner: ComRc<dyn ICoreWebView2CookieManager>,
}
impl From<ComRc<dyn ICoreWebView2CookieManager>> for CookieManager {
    fn from(inner: ComRc<dyn ICoreWebView2CookieManager>) -> Self {
        Self { inner }
    }
}
impl fmt::Debug for CookieManager {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("CookieManager").finish()
    }
}
impl CookieManager {
    pub fn into_inner(self) -> ComRc<dyn ICoreWebView2CookieManager> {
        self.inner
    }
    pub fn as_inner(&self) -> &ComRc<dyn ICoreWebView2CookieManager> {
        &self.inner
    }
}

/// Wrapper for `ICoreWebView2CookieList`.
#[derive(Clone)]
pub struct CookieList {
    inner: ComRc<dyn ICoreWebView2CookieList>,
}
impl From<ComRc<dyn ICoreWebView2CookieList>> for CookieList {
    fn from(inner: ComRc<dyn ICoreWebView2CookieList>) -> Self {
        Self { inner }
    }
}
impl fmt::Debug for CookieList {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("CookieList").finish()
    }
}
impl CookieList {
    pub fn into_inner(self) -> ComRc<dyn ICoreWebView2CookieList> {
        self.inner
    }
    pub fn as_inner(&self) -> &ComRc<dyn ICoreWebView2CookieList> {
        &self.inner
    }
}

/// Wrapper for `ICoreWebView2PointerInfo`.
#[derive(Clone)]
pub struct PointerInfo {
    inner: ComRc<dyn ICoreWebView2PointerInfo>,
}
impl From<ComRc<dyn ICoreWebView2PointerInfo>> for PointerInfo {
    fn from(inner: ComRc<dyn ICoreWebView2PointerInfo>) -> Self {
        Self { inner }
    }
}
impl fmt::Debug for PointerInfo {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("PointerInfo").finish()
    }
}
impl PointerInfo {
    pub fn into_inner(self) -> ComRc<dyn ICoreWebView2PointerInfo> {
        self.inner
    }
    pub fn as_inner(&self) -> &ComRc<dyn ICoreWebView2PointerInfo> {
        &self.inner
    }
}

/// Wrapper for `ICoreWebView2Environment`.
#[derive(Clone)]
pub struct Environment {
    inner: ComRc<dyn ICoreWebView2Environment>,
}
impl From<ComRc<dyn ICoreWebView2Environment>> for Environment {
    fn from(inner: ComRc<dyn ICoreWebView2Environment>) -> Self {
        Self { inner }
    }
}
impl fmt::Debug for Environment {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Environment").finish()
    }
}
impl Environment {
    pub fn into_inner(self) -> ComRc<dyn ICoreWebView2Environment> {
        self.inner
    }
    pub fn as_inner(&self) -> &ComRc<dyn ICoreWebView2Environment> {
        &self.inner
    }
}

/// Wrapper for `ICoreWebView2Environment2`.
#[derive(Clone)]
pub struct Environment2 {
    inner: ComRc<dyn ICoreWebView2Environment2>,
}
impl From<ComRc<dyn ICoreWebView2Environment2>> for Environment2 {
    fn from(inner: ComRc<dyn ICoreWebView2Environment2>) -> Self {
        Self { inner }
    }
}
impl fmt::Debug for Environment2 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Environment2").finish()
    }
}
impl Environment2 {
    pub fn into_inner(self) -> ComRc<dyn ICoreWebView2Environment2> {
        self.inner
    }
    pub fn as_inner(&self) -> &ComRc<dyn ICoreWebView2Environment2> {
        &self.inner
    }
}

/// Wrapper for `ICoreWebView2Environment3`.
#[derive(Clone)]
pub struct Environment3 {
    inner: ComRc<dyn ICoreWebView2Environment3>,
}
impl From<ComRc<dyn ICoreWebView2Environment3>> for Environment3 {
    fn from(inner: ComRc<dyn ICoreWebView2Environment3>) -> Self {
        Self { inner }
    }
}
impl fmt::Debug for Environment3 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Environment3").finish()
    }
}
impl Environment3 {
    pub fn into_inner(self) -> ComRc<dyn ICoreWebView2Environment3> {
        self.inner
    }
    pub fn as_inner(&self) -> &ComRc<dyn ICoreWebView2Environment3> {
        &self.inner
    }
}

/// Wrapper for `ICoreWebView2Environment4`.
#[derive(Clone)]
pub struct Environment4 {
    inner: ComRc<dyn ICoreWebView2Environment4>,
}
impl From<ComRc<dyn ICoreWebView2Environment4>> for Environment4 {
    fn from(inner: ComRc<dyn ICoreWebView2Environment4>) -> Self {
        Self { inner }
    }
}
impl fmt::Debug for Environment4 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Environment4").finish()
    }
}
impl Environment4 {
    pub fn into_inner(self) -> ComRc<dyn ICoreWebView2Environment4> {
        self.inner
    }
    pub fn as_inner(&self) -> &ComRc<dyn ICoreWebView2Environment4> {
        &self.inner
    }
}

/// Wrapper for `ICoreWebView2EnvironmentOptions`.
#[derive(Clone)]
pub struct EnvironmentOptions {
    inner: ComRc<dyn ICoreWebView2EnvironmentOptions>,
}
impl From<ComRc<dyn ICoreWebView2EnvironmentOptions>> for EnvironmentOptions {
    fn from(inner: ComRc<dyn ICoreWebView2EnvironmentOptions>) -> Self {
        Self { inner }
    }
}
impl fmt::Debug for EnvironmentOptions {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("EnvironmentOptions").finish()
    }
}
impl EnvironmentOptions {
    pub fn into_inner(self) -> ComRc<dyn ICoreWebView2EnvironmentOptions> {
        self.inner
    }
    pub fn as_inner(&self) -> &ComRc<dyn ICoreWebView2EnvironmentOptions> {
        &self.inner
    }
}

/// Wrapper for `ICoreWebView2DevToolsProtocolEventReceiver`.
#[derive(Clone)]
pub struct DevToolsProtocolEventReceiver {
    inner: ComRc<dyn ICoreWebView2DevToolsProtocolEventReceiver>,
}
impl From<ComRc<dyn ICoreWebView2DevToolsProtocolEventReceiver>> for DevToolsProtocolEventReceiver {
    fn from(inner: ComRc<dyn ICoreWebView2DevToolsProtocolEventReceiver>) -> Self {
        Self { inner }
    }
}
impl fmt::Debug for DevToolsProtocolEventReceiver {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("DevToolsProtocolEventReceiver").finish()
    }
}
impl DevToolsProtocolEventReceiver {
    pub fn into_inner(self) -> ComRc<dyn ICoreWebView2DevToolsProtocolEventReceiver> {
        self.inner
    }
    pub fn as_inner(&self) -> &ComRc<dyn ICoreWebView2DevToolsProtocolEventReceiver> {
        &self.inner
    }
}

/// Wrapper for `ICoreWebView2ProcessFailedEventArgs2`.
#[derive(Clone)]
pub struct ProcessFailedEventArgs2 {
    inner: ComRc<dyn ICoreWebView2ProcessFailedEventArgs2>,
}
impl From<ComRc<dyn ICoreWebView2ProcessFailedEventArgs2>> for ProcessFailedEventArgs2 {
    fn from(inner: ComRc<dyn ICoreWebView2ProcessFailedEventArgs2>) -> Self {
        Self { inner }
    }
}
impl fmt::Debug for ProcessFailedEventArgs2 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("ProcessFailedEventArgs2").finish()
    }
}
impl ProcessFailedEventArgs2 {
    pub fn into_inner(self) -> ComRc<dyn ICoreWebView2ProcessFailedEventArgs2> {
        self.inner
    }
    pub fn as_inner(&self) -> &ComRc<dyn ICoreWebView2ProcessFailedEventArgs2> {
        &self.inner
    }
}

/// Wrapper for `ICoreWebView2FrameInfoCollection`.
#[derive(Clone)]
pub struct FrameInfoCollection {
    inner: ComRc<dyn ICoreWebView2FrameInfoCollection>,
}
impl From<ComRc<dyn ICoreWebView2FrameInfoCollection>> for FrameInfoCollection {
    fn from(inner: ComRc<dyn ICoreWebView2FrameInfoCollection>) -> Self {
        Self { inner }
    }
}
impl fmt::Debug for FrameInfoCollection {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("FrameInfoCollection").finish()
    }
}
impl FrameInfoCollection {
    pub fn into_inner(self) -> ComRc<dyn ICoreWebView2FrameInfoCollection> {
        self.inner
    }
    pub fn as_inner(&self) -> &ComRc<dyn ICoreWebView2FrameInfoCollection> {
        &self.inner
    }
}

/// Wrapper for `ICoreWebView2FrameInfoCollectionIterator`.
#[derive(Clone)]
pub struct FrameInfoCollectionIterator {
    inner: ComRc<dyn ICoreWebView2FrameInfoCollectionIterator>,
}
impl From<ComRc<dyn ICoreWebView2FrameInfoCollectionIterator>> for FrameInfoCollectionIterator {
    fn from(inner: ComRc<dyn ICoreWebView2FrameInfoCollectionIterator>) -> Self {
        Self { inner }
    }
}
impl fmt::Debug for FrameInfoCollectionIterator {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("FrameInfoCollectionIterator").finish()
    }
}
impl FrameInfoCollectionIterator {
    pub fn into_inner(self) -> ComRc<dyn ICoreWebView2FrameInfoCollectionIterator> {
        self.inner
    }
    pub fn as_inner(&self) -> &ComRc<dyn ICoreWebView2FrameInfoCollectionIterator> {
        &self.inner
    }
}

/// Wrapper for `ICoreWebView2FrameInfo`.
#[derive(Clone)]
pub struct FrameInfo {
    inner: ComRc<dyn ICoreWebView2FrameInfo>,
}
impl From<ComRc<dyn ICoreWebView2FrameInfo>> for FrameInfo {
    fn from(inner: ComRc<dyn ICoreWebView2FrameInfo>) -> Self {
        Self { inner }
    }
}
impl fmt::Debug for FrameInfo {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("FrameInfo").finish()
    }
}
impl FrameInfo {
    pub fn into_inner(self) -> ComRc<dyn ICoreWebView2FrameInfo> {
        self.inner
    }
    pub fn as_inner(&self) -> &ComRc<dyn ICoreWebView2FrameInfo> {
        &self.inner
    }
}

/// Wrapper for `ICoreWebView2Interop`.
#[derive(Clone)]
pub struct Interop {
    inner: ComRc<dyn ICoreWebView2Interop>,
}
impl From<ComRc<dyn ICoreWebView2Interop>> for Interop {
    fn from(inner: ComRc<dyn ICoreWebView2Interop>) -> Self {
        Self { inner }
    }
}
impl fmt::Debug for Interop {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Interop").finish()
    }
}
impl Interop {
    pub fn into_inner(self) -> ComRc<dyn ICoreWebView2Interop> {
        self.inner
    }
    pub fn as_inner(&self) -> &ComRc<dyn ICoreWebView2Interop> {
        &self.inner
    }
}

/// Wrapper for `ICoreWebView2CompositionControllerInterop`.
#[derive(Clone)]
pub struct CompositionControllerInterop {
    inner: ComRc<dyn ICoreWebView2CompositionControllerInterop>,
}
impl From<ComRc<dyn ICoreWebView2CompositionControllerInterop>> for CompositionControllerInterop {
    fn from(inner: ComRc<dyn ICoreWebView2CompositionControllerInterop>) -> Self {
        Self { inner }
    }
}
impl fmt::Debug for CompositionControllerInterop {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("CompositionControllerInterop").finish()
    }
}
impl CompositionControllerInterop {
    pub fn into_inner(self) -> ComRc<dyn ICoreWebView2CompositionControllerInterop> {
        self.inner
    }
    pub fn as_inner(&self) -> &ComRc<dyn ICoreWebView2CompositionControllerInterop> {
        &self.inner
    }
}

/// Wrapper for `ICoreWebView2EnvironmentInterop`.
#[derive(Clone)]
pub struct EnvironmentInterop {
    inner: ComRc<dyn ICoreWebView2EnvironmentInterop>,
}
impl From<ComRc<dyn ICoreWebView2EnvironmentInterop>> for EnvironmentInterop {
    fn from(inner: ComRc<dyn ICoreWebView2EnvironmentInterop>) -> Self {
        Self { inner }
    }
}
impl fmt::Debug for EnvironmentInterop {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("EnvironmentInterop").finish()
    }
}
impl EnvironmentInterop {
    pub fn into_inner(self) -> ComRc<dyn ICoreWebView2EnvironmentInterop> {
        self.inner
    }
    pub fn as_inner(&self) -> &ComRc<dyn ICoreWebView2EnvironmentInterop> {
        &self.inner
    }
}

/// Wrapper for `IStream`.
#[derive(Clone)]
pub struct Stream {
    inner: ComRc<dyn IStream>,
}
impl From<ComRc<dyn IStream>> for Stream {
    fn from(inner: ComRc<dyn IStream>) -> Self {
        Self { inner }
    }
}
impl fmt::Debug for Stream {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Stream").finish()
    }
}
impl Stream {
    pub fn into_inner(self) -> ComRc<dyn IStream> {
        self.inner
    }
    pub fn as_inner(&self) -> &ComRc<dyn IStream> {
        &self.inner
    }
}
