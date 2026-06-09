declare class EngineObject {
    get name(): string;
}
declare abstract class Component extends EngineObject {
    private _handle;
    private _transform;
    private _go;
    get instanceID(): number;
    get transform(): Transform;
    get gameObject(): GameObject;
    get enable(): boolean;
    get allowEnable(): boolean;
    set enable(enable: boolean);
    toString(): string;
    private static get_instance_id_impl;
    private static get_transform_impl;
    private static get_go_impl;
    private static get_enable_impl;
    private static get_allow_enable_impl;
    private static set_enable_impl;
    private static get_type_name_impl;
}
interface IBehaviorTask {
    OnAwake(): void;
    OnStart(): void;
    OnUpdate(): BehaviorTaskStatus;
    OnFixedUpdate(): void;
    OnEnd(): void;
    OnPause(paused: boolean): any;
    GetUtility(): number;
    OnBehaviorComplete(): void;
    OnReset(): void;
    CanReevaluate(): boolean;
    OnReevaluate(): void;
}
declare abstract class BehaviorTask implements IBehaviorTask {
    private _handle;
    constructor(handle: never);
    get Owner(): BehaviorTree;
    get ID(): number;
    get FriendlyName(): string;
    get IsInstant(): boolean;
    get Disabled(): boolean;
    set FriendlyName(name: string);
    set IsInstant(val: boolean);
    set Disabled(val: boolean);
    OnAwake(): void;
    OnStart(): void;
    OnUpdate(): BehaviorTaskStatus;
    OnFixedUpdate(): void;
    OnEnd(): void;
    OnPause(paused: boolean): void;
    GetUtility(): number;
    OnBehaviorComplete(): void;
    OnReset(): void;
    CanReevaluate(): boolean;
    OnReevaluate(): void;
    GetStatus(): BehaviorTaskStatus;
    GetPriority(): number;
    private static base_handle_check;
    private static base_alloc_object;
    private static null_handle_check;
    private static get_status;
    private static get_id;
    private static get_name;
    private static get_disable;
    private static get_instant;
    private static get_owner;
    private static set_name;
    private static set_disable;
    private static set_instant;
    private static on_awake;
    private static on_start;
    private static on_update;
    private static on_fixedUpdate;
    private static on_end;
    private static on_pause;
    private static on_utility;
    private static on_behaviorComplete;
    private static on_reset;
    private static can_reevaluate;
    private static on_reevaluate;
}
interface IOnEnable {
    OnEnable(): void;
}
interface IOnStart {
    OnStart(): void;
}
interface IOnTriggerEnter {
    OnTriggerEnter(collider: Collider): void;
}
interface IOnTriggerStay {
    OnTriggerStay(collider: Collider): void;
}
interface IOnTriggerExit {
    OnTriggerExit(collider: Collider): void;
}
interface IOnCollisionEnter {
    OnCollisionEnter(collision: Collision): void;
}
interface IOnCollisionStay {
    OnCollisionStay(collision: Collision): void;
}
interface IOnCollisionExit {
    OnCollisionExit(collision: Collision): void;
}
interface IOnControllerColliderHit {
    OnControllerColliderHit(collision: ControllerColliderHit): void;
}
interface IOnFixedUpdate {
    OnFixedUpdate(): void;
}
interface IOnUpdate {
    OnUpdate(): void;
}
interface IOnLateUpdate {
    OnLateUpdate(): void;
}
interface IOnPreCull {
    OnPreCull(): void;
}
interface IOnPreRender {
    OnPreRender(): void;
}
interface IOnRenderObject {
    OnRenderObject(): void;
}
interface IOnPostRender {
    OnPostRender(): void;
}
interface IOnRenderImage {
    OnRenderImage(src: RenderTexture, dst: RenderTexture): void;
}
interface IOnDrawGizmos {
    OnDrawGizmos(): void;
}
interface IOnDrawGizmosSelected {
    OnDrawGizmosSelected(): void;
}
interface IOnGUI {
    OnGUI(): void;
}
interface IOnDisable {
    OnDisable(): void;
}
interface IOnDestroy {
    OnDestroy(): void;
}
interface IOnInvoke {
    OnInvoke(): void;
}
interface IOnLastRender {
    OnLastRender(): void;
}
interface IBehaviorParentTask {
    CanRunParallelChildren(): boolean;
    CurrentChildIndex(): number;
    CanExecute(): boolean;
    OnChildExecuted(child_idx: number, child_status: BehaviorTaskStatus): void;
    OnChildStarted(child_idx: number): void;
    OverrideStatus(): BehaviorTaskStatus;
}
declare abstract class AssetObject extends EngineObject {
    private _handle;
    get assetType(): AssetType;
    get name(): string;
    set name(name: string);
    toString(): string;
    Cast<T extends AssetObject>(type: Traits_Constructor<T>): T;
    private static get_type_impl;
    private static get_name_impl;
    private static set_name_impl;
    private static get_type_name_impl;
    private static castable_impl;
}
declare abstract class BehaviorParentTask extends BehaviorTask implements IBehaviorParentTask {
    constructor(handle: never);
    CanRunParallelChildren(): boolean;
    CurrentChildIndex(): number;
    CanExecute(): boolean;
    OnChildExecuted(child_idx: number, child_status: BehaviorTaskStatus): void;
    OnChildStarted(child_idx: number): void;
    OverrideStatus(): BehaviorTaskStatus;
    GetChildrenList(): BehaviorTask[];
    GetChildrenCount(): number;
    AddChild(child: BehaviorTask): void;
    ReplaceAddChild(child: BehaviorTask, idx: number): void;
    private static add_child;
    private static get_children_list;
    private static get_children_cnt;
    private static replace_child;
    private static can_run_parallel;
    private static current_child_idx;
    private static can_exec;
    private static child_started;
    private static child_exec;
    private static override_status;
}
declare abstract class Control extends EngineObject {
    private _handle;
    get instanceID(): number;
    get name(): string;
    get type(): ControlType;
    get rectTransform(): RectTransform;
    get isEnable(): boolean;
    get isShown(): boolean;
    get form(): Form | null;
    get canvas(): Canvas | null;
    get childCount(): number;
    get guiEventData(): GUIEventData;
    set name(name: string);
    GetParent<T extends Control>(type?: Traits_Constructor<T>): T | null;
    SetParent(parent: Control | null, keepWorldPos?: boolean): void;
    AddChild(child: Control): void;
    RemoveChild(index: number): void;
    ClearAllChildren(): void;
    GetChildByIndex(index: number): Control;
    FindChild<T extends Control | LayoutGroup>(type: Traits_Constructor<T>, name: string): T | null;
    Close(): void;
    Show(): void;
    Hide(): void;
    Enable(): void;
    Disable(): void;
    Clone<T extends Control>(type: Traits_Constructor<T>, parent?: Control | null): T;
    GetLayoutGroup<T extends LayoutGroup>(type?: Traits_Constructor<T>): T | null;
    GetLayoutGroupSize(): Vector2;
    ResetLayoutGroup(): void;
    abstract AddEvent(event: ControlBaseEvent | ControlEvent | number, callBack: IControlEventCallBack | IControlEventCallBackI | IControlEventCallBackS): void;
    protected _AddEvent(event: number, callBack: IControlEventCallBack | IControlEventCallBackI | IControlEventCallBackS): void;
    abstract ClearEvent(event: ControlBaseEvent | ControlEvent | number, callBack?: IControlEventCallBack | IControlEventCallBackI | IControlEventCallBackS | null): void;
    protected _ClearEvent(event: number, callBack?: IControlEventCallBack | IControlEventCallBackI | IControlEventCallBackS | null): void;
    toString(): string;
    private static get_instance_id;
    private static get_name;
    private static get_type;
    private static get_rect_transform;
    private static get_enable;
    private static get_shown;
    private static get_form;
    private static get_canvas;
    private static get_child_count;
    private static get_child_by_index;
    private static get_gui_event_data;
    private static set_name;
    private static get_parent;
    private static set_parent;
    private static add_child;
    private static remove_child;
    private static clear_child;
    private static find_child;
    private static close;
    private static show;
    private static hide;
    private static enable;
    private static disable;
    private static add_event;
    private static clear_event;
    private static clone;
    private static get_layout_group;
    private static get_layout_group_size;
    private static reset_layout_group;
}
interface IBehaviorDecorator {
    Decorate(status: BehaviorTaskStatus): BehaviorTaskStatus;
}
declare abstract class BehaviorComposite extends BehaviorParentTask implements IBehaviorComposite {
    constructor(handle?: never);
    get AbortType(): BehaviorAbortType;
    set AbortType(abort_type: BehaviorAbortType);
    OnConditionalAbort(child_idx: number): void;
    private static get_abort_type;
    private static set_abort_type;
    private static on_condition_abort;
}
interface IBehaviorComposite {
    OnConditionalAbort(child_idx: number): void;
}
declare abstract class BehaviorDecorator extends BehaviorParentTask implements IBehaviorDecorator {
    constructor(handle?: never);
    Decorate(status: BehaviorTaskStatus): BehaviorTaskStatus;
    private static decorate;
}
declare class RongCloud {
    private static _dispatchList;
    private static _responseList;
    private static _platUserId;
    static Init(platId: number): void;
    static Release(): void;
    static Update(): void;
    static OnSendMessage(receiverPlatId: string, messageType: number, content: string): void;
    static OnGetTalkList(timeStamp: number, count: number): void;
    static OnGetTalkDetail(targetId: string, lastMessageId: number, count: number): void;
    static ClearTalkDetail(targetId: string): void;
    static RemoveMessage(messageId: number): void;
    static OnPlayVoice(uri: string): void;
    static OnStartRecordVoice(receiverPlatId: string): void;
    static OnStopRecordVoice(receiverPlatId: string, isCancel: boolean): void;
    private static eventReceiveMessageCallBack;
    static set EventReceiveMessage(callback: (message: RongCloudMessage) => void);
    private static eventOnTalkListCallBack;
    static set EventOnTalkList(callback: (data: RongCloudTalkListData) => void);
    private static eventOnTalkDetailCallBack;
    static set EventOnTalkDetail(callback: (data: RongCloudTalkDetailData) => void);
    private static eventOnSendMsgErrorCallBack;
    static set EventOnSendMsgError(callback: IRongCloudOnSendMsgError);
    private static eventOnReceiveNotificationCallBack;
    static set EventOnReceiveNotification(callback: IRongCloudOnReceiveNotification);
    private static event_receive_message;
    private static event_on_talk_list;
    private static event_on_talk_detail;
    private static event_on_send_msg_error;
    private static event_on_receive_notification;
}
declare enum RongCloudWaitType {
    None = 0,
    Self = 1,
    Dispatch = 2,
    Response = 3
}
declare enum RongCloudMessageType {
    TXT_MESSAGE = 0,
    IMG_MESSAGE = 1,
    VOICE_MESSAGE = 2,
    VIDEO_MESSAGE = 3,
    EXPRESSION_MESSAGE = 4,
    INVITE_PLAY_GAME = 5,
    INVITE_PLAY_GAME_BED_WAR = 6
}
declare abstract class RongCloudResponse {
    func: Function;
    Do(): string;
    abstract Response(): void;
}
declare class RongCloudTalkListData extends RongCloudResponse {
    timeStamp: number;
    talkList: RongCloudTalk[];
    Response(): void;
}
declare class RongCloudTalk {
    conversationType: string;
    latestMessage: RongCloudMessage;
    latestMessageId: number;
    portraitUrl: string;
    receivedTime: number;
    senderUserId: string;
    sentTime: number;
    targetId: string;
    unreadMessageCount: number;
}
declare class RongCloudTalkDetailData extends RongCloudResponse {
    targetId: string;
    details: RongCloudMessage[];
    Response(): void;
}
declare class RongCloudMessage extends RongCloudResponse {
    content: string;
    duration: number;
    uri: string;
    messageId: number;
    messageType: number;
    receivedTime: number;
    receiverUserId: string;
    senderUserId: string;
    sentTime: number;
    sourceType: string;
    constructor(content?: string);
    Response(): void;
}
declare class RongCloudDispatch {
    type: string;
    invokeName: string;
    otherPlatId: string;
    messageType: number;
    content: string;
    lastMessageId: number;
    count: number;
    isCancel: boolean;
    constructor(type: string, invokeName: string);
    Do(platId: string): string;
    Dispatch(platId: string): void;
}
declare abstract class BehaviorAction extends BehaviorTask {
    constructor(handle?: never);
}
declare abstract class IKSolver extends Component {
    get weight(): number;
    set weight(IKweight: number);
    private static getWeight;
    private static setWeight;
}
declare abstract class Renderer extends Component {
    get rendererPriority(): number;
    get material(): Material;
    get sharedMaterial(): Material;
    get forceRenderingOff(): boolean;
    get localBounds(): Bounds;
    get isVisible(): boolean;
    get shadowCastingMode(): ShadowCastingMode;
    get receiveShadows(): boolean;
    get bounds(): Bounds;
    get lastFrameDrawed(): boolean;
    get materials(): Material[] | null;
    get sharedMaterials(): Material[] | null;
    set rendererPriority(value: number);
    set material(value: Material);
    set materials(value: Material[]);
    set sharedMaterial(value: Material);
    set sharedMaterials(value: Material[]);
    set forceRenderingOff(value: boolean);
    set localBounds(value: Bounds);
    set shadowCastingMode(value: ShadowCastingMode);
    set receiveShadows(value: boolean);
    set bounds(value: Bounds);
    private static getRendererPriority_impl;
    private static getMaterial_impl;
    private static getMaterials_impl;
    private static getSharedMaterial_impl;
    private static getSharedMaterials_impl;
    private static getForceRenderingOff_impl;
    private static getLocalBounds_impl;
    private static getIsVisible_impl;
    private static getShadowCastingMode_impl;
    private static getReceiveShadows_impl;
    private static getBounds_impl;
    private static getLastFrameDrawed_impl;
    private static setRendererPriority_impl;
    private static setMaterial_impl;
    private static setMaterials_impl;
    private static setSharedMaterial_impl;
    private static setSharedMaterials_impl;
    private static setForceRenderingOff_impl;
    private static setLocalBounds_impl;
    private static setShadowCastingMode_impl;
    private static setReceiveShadows_impl;
    private static setBounds_impl;
}
declare abstract class TransitionControl extends Control {
    get interactable(): boolean;
    get transitionMode(): TransitionMode;
    get colorMultiplier(): number;
    get fadeDuration(): number;
    get normalColor(): Color;
    get highlightedColor(): Color;
    get pressedColor(): Color;
    get selectedColor(): Color;
    get dissabledColor(): Color;
    get normalTexture(): Texture;
    get highlightedTexture(): Texture;
    get pressedTexture(): Texture;
    get selectedTexture(): Texture;
    get dissabledTexture(): Texture;
    get normalSprite(): Sprite;
    get highlightedSprite(): Sprite;
    get pressedSprite(): Sprite;
    get selectedSprite(): Sprite;
    get dissabledSprite(): Sprite;
    get normalSpriteSequence(): SpriteSequence;
    get highlightedSpriteSequence(): SpriteSequence;
    get pressedSpriteSequence(): SpriteSequence;
    get selectedSpriteSequence(): SpriteSequence;
    get dissabledSpriteSequence(): SpriteSequence;
    set interactable(enable: boolean);
    set transitionMode(mode: TransitionMode);
    set colorMultiplier(multiplier: number);
    set fadeDuration(duration: number);
    set normalColor(color: Color);
    set highlightedColor(color: Color);
    set pressedColor(color: Color);
    set selectedColor(color: Color);
    set dissabledColor(color: Color);
    set normalTexture(texture: Texture | null);
    set highlightedTexture(texture: Texture | null);
    set pressedTexture(texture: Texture | null);
    set selectedTexture(texture: Texture | null);
    set dissabledTexture(texture: Texture | null);
    set normalSprite(sprite: Sprite | null);
    set highlightedSprite(sprite: Sprite | null);
    set pressedSprite(sprite: Sprite | null);
    set selectedSprite(sprite: Sprite | null);
    set dissabledSprite(sprite: Sprite | null);
    set normalSpriteSequence(spriteSequence: SpriteSequence | null);
    set highlightedSpriteSequence(spriteSequence: SpriteSequence | null);
    set pressedSpriteSequence(spriteSequence: SpriteSequence | null);
    set selectedSpriteSequence(spriteSequence: SpriteSequence | null);
    set dissabledSpriteSequence(spriteSequence: SpriteSequence | null);
    private static get_interactable;
    private static get_transition_mode;
    private static get_color_multiplier;
    private static get_fade_duration;
    private static get_normal_color;
    private static get_highlighted_color;
    private static get_pressed_color;
    private static get_selected_color;
    private static get_dissabled_color;
    private static get_normal_texture;
    private static get_highlighted_texture;
    private static get_pressed_texture;
    private static get_selected_texture;
    private static get_dissabled_texture;
    private static get_normal_sprite;
    private static get_highlighted_sprite;
    private static get_pressed_sprite;
    private static get_selected_sprite;
    private static get_dissabled_sprite;
    private static get_normal_sprite_sequence;
    private static get_highlighted_sprite_sequence;
    private static get_pressed_sprite_sequence;
    private static get_selected_sprite_sequence;
    private static get_dissabled_sprite_sequence;
    private static set_interactable;
    private static set_transition_mode;
    private static set_color_multiplier;
    private static set_fade_duration;
    private static set_normal_color;
    private static set_highlighted_color;
    private static set_pressed_color;
    private static set_selected_color;
    private static set_dissabled_color;
    private static set_normal_texture;
    private static set_highlighted_texture;
    private static set_pressed_texture;
    private static set_selected_texture;
    private static set_dissabled_texture;
    private static set_normal_sprite;
    private static set_highlighted_sprite;
    private static set_pressed_sprite;
    private static set_selected_sprite;
    private static set_dissabled_sprite;
    private static set_normal_sprite_sequence;
    private static set_highlighted_sprite_sequence;
    private static set_pressed_sprite_sequence;
    private static set_selected_sprite_sequence;
    private static set_dissabled_sprite_sequence;
}
declare abstract class Collider extends Component {
    get bounds(): Bounds;
    get isTrigger(): Boolean;
    get contactOffset(): number;
    get attachedRigidbody(): Rigidbody;
    get sharedMaterial(): PhysicMaterial;
    get material(): PhysicMaterial;
    set isTrigger(isTrigger: Boolean);
    set contactOffset(offset: number);
    set sharedMaterial(mat: PhysicMaterial);
    set material(mat: PhysicMaterial);
    ClosestPoint(position: Vector3): Vector3;
    ClosestPointOnBounds(position: Vector3): Vector3;
    Raycast(ray: Ray, maxDistance?: number): RaycastHit | undefined;
    private static getBounds;
    private static getIsTrigger;
    private static getContactOffset;
    private static getAttachedRigidbody;
    private static getSharedMaterial;
    private static getMaterial;
    private static setIsTrigger;
    private static setContactOffset;
    private static setSharedMaterial;
    private static setMaterial;
    private static closest_point_impl;
    private static closest_point_onbounds_impl;
    private static raycast_impl;
}
declare abstract class BehaviorVariable {
    private _handle;
    constructor(handle?: never);
    abstract GetValue(): any;
    abstract SetValue(val: any): void;
    get Name(): string;
    private static AllocHandle;
    private static alloc_object_var;
    private static handle_check_var;
    private static null_handle_check;
    private static get_name;
    private static set_name;
}
interface IRongCloudReceiveMessage {
    (sourceType: string, messageType: number, content: string): void;
}
interface IRongCloudOnTalkList {
    (timeStamp: number, talkList: string): void;
}
interface IRongCloudOnTalkDetail {
    (targetId: string, details: string): void;
}
interface IRongCloudOnSendMsgError {
    (receiverUserId: string, code: number, errMsg: string): void;
}
interface IRongCloudOnReceiveNotification {
    (contactNotificationMessage: string): void;
}
declare abstract class PostEffectBase extends EngineObject {
    private _handle;
    get postEffectDefine(): PostEffectDefine;
    get sortIndex(): number;
    get enable(): boolean;
    set enable(value: boolean);
    private static getPostEffectDefine_impl;
    private static SortIndex_impl;
    private static getEnable_impl;
    private static setEnable_impl;
}
declare abstract class Texture extends AssetObject {
    get isReadable(): boolean;
    get width(): number;
    get height(): number;
    get dimension(): TextureDimension;
    get graphicFormat(): GraphicsFormat;
    get anisoLevel(): AnisoLevel;
    get mipmapCount(): number;
    get texelSize(): Vector2;
    get mipmapBias(): number;
    get wrapMode(): TextureWrapMode;
    get wrapModeU(): TextureWrapMode;
    get wrapModeV(): TextureWrapMode;
    get wrapModeW(): TextureWrapMode;
    get filterMode(): FilterMode;
    get isCrash(): boolean;
    set anisoLevel(value: AnisoLevel);
    set mipmapBias(value: number);
    set filterMode(value: FilterMode);
    set wrapMode(value: TextureWrapMode);
    set wrapModeU(value: TextureWrapMode);
    set wrapModeV(value: TextureWrapMode);
    set wrapModeW(value: TextureWrapMode);
    GenerateMipmaps(): boolean;
    ReleaseMipmaps(): void;
    private static is_readable_impl;
    private static get_width_impl;
    private static get_height_impl;
    private static get_dimension_impl;
    private static get_graphic_format_impl;
    private static get_aniso_level_impl;
    private static get_mip_map_count_impl;
    private static get_texel_size_impl;
    private static get_mipmap_bias_impl;
    private static get_wrap_mode_impl;
    private static get_wrap_mode_u_impl;
    private static get_wrap_mode_v_impl;
    private static get_wrap_mode_w_impl;
    private static get_filter_mode_impl;
    private static get_is_crash_impl;
    private static set_aniso_level_impl;
    private static set_mipmap_bias_impl;
    private static set_wrap_mode_impl;
    private static set_wrap_mode_u_impl;
    private static set_wrap_mode_v_impl;
    private static set_wrap_mode_w_impl;
    private static set_filter_mode_impl;
    private static gen_mipmaps_impl;
    private static release_mipmaps_impl;
}
interface IContracter {
    FindEntity(entityID: string): NetEntity;
    CreateNewEntity(prefab_id: string, userdata: string, callback?: (entity: NetEntity) => void): void;
    CurrentConversation(): NetConversation;
}
declare abstract class LayoutGroup extends EngineObject {
    private _handle;
    get layoutGroupType(): LayoutGroupType;
    get layoutAlignment(): LayoutAlignmentType;
    get left(): number;
    get right(): number;
    get top(): number;
    get bottom(): number;
    get limitSize(): boolean;
    get childSize(): Vector2;
    get autoResize(): boolean;
    get control(): Control;
    set layoutAlignment(alignment: LayoutAlignmentType);
    set left(left: number);
    set right(right: number);
    set top(top: number);
    set bottom(bottom: number);
    set limitSize(enable: boolean);
    set childSize(size: Vector2);
    set autoResize(value: boolean);
    toString(): string;
    private static get_layout_group_type;
    private static get_layout_alignment;
    private static get_left;
    private static get_right;
    private static get_top;
    private static get_bottom;
    private static get_limitSize;
    private static get_child_size;
    private static get_auto_resize;
    private static get_control;
    private static set_layout_alignment;
    private static set_left;
    private static set_right;
    private static set_top;
    private static set_bottom;
    private static set_limit_size;
    private static set_child_size;
    private static set_auto_resize;
}
interface ITweenerCallBack {
    (): void;
}
interface ITweenCallBack {
    (value: TweenOperator): void;
}
interface IEaseFunction {
    (t: number, b: number, c: number, d: number): number;
}
interface IControlEventCallBack {
    (control: Control): void;
}
interface IControlEventCallBackI {
    (control: Control, trigger: number): void;
}
interface IControlEventCallBackS {
    (control: Control, event_string: String): void;
}
interface IVirtualItemCreateCallBack {
    (item: LayoutItem): void;
}
interface IVirtualItemRenderCallBack {
    (item: LayoutItem): void;
}
interface IVirtualItemRecycleCallBack {
    (item: LayoutItem): void;
}
declare abstract class TweenOperator extends EngineObject {
    protected _handle: IntPtr;
    constructor(handle: never);
    abstract TimeScale(value: number): TweenOperator;
    abstract LoopCount(value: number): TweenOperator;
    abstract AddStartCallBack(value: ITweenCallBack): TweenOperator;
    abstract AddUpdateCallBack(value: ITweenCallBack): TweenOperator;
    abstract AddStepCompletedCallBack(value: ITweenCallBack): TweenOperator;
    abstract AddFinishCallBack(value: ITweenCallBack): TweenOperator;
    abstract Invalid(): boolean;
    Pause(): void;
    Continue(): void;
    Restart(): void;
    Destroy(): void;
    private _invalid;
    protected static _handle_check(handle: IntPtr): boolean;
    protected static set_time_scale(handle: IntPtr, value: number): void;
    protected static set_loop_count(handle: IntPtr, value: number): void;
    protected static add_start_call_back(handle: IntPtr, value: ITweenCallBack): void;
    protected static add_update_call_back(handle: IntPtr, value: ITweenCallBack): void;
    protected static add_step_completed_call_back(handle: IntPtr, value: ITweenCallBack): void;
    protected static add_finish_call_back(handle: IntPtr, value: ITweenCallBack): void;
    protected static pause(handle: IntPtr): void;
    protected static continue(handle: IntPtr): void;
    protected static restart(handle: IntPtr): void;
    protected static destroy(handle: IntPtr): void;
}
declare class NavMeshHit {
    private _distance;
    private _hit;
    private _mask;
    private _normal;
    private _position;
    get distance(): number;
    get hit(): boolean;
    get mask(): number;
    get normal(): Vector3;
    get position(): Vector3;
    set distance(val: number);
    set hit(val: boolean);
    set mask(val: number);
    set normal(val: Vector3);
    set position(val: Vector3);
}
interface NavMeshHitUser {
}
declare enum JsonFormat {
    Minify = 1,
    Normal = 2
}
declare class Reflection {
    private static GetType;
    private static GetIsPrivate;
}
declare class JsonSettings {
    static __DisableJsonMember: {};
    static __JsonMember: {};
    static __jsonClass: {};
    static __init: boolean;
    static __reflection: any;
    static Init(): void;
    static JsonMember(target: any, propertyKey: string): any;
    static JsonClass(constructor: any): any;
    static DisableJsonMember(target: any, propertyKey: string): void;
}
declare class JsonHelper {
    static ToJson(obj: any, isSerialized?: boolean, format?: JsonFormat, replacer?: (this: any, key: string, value: any) => any | (number | string)[] | null): string;
    static FromJson(jsonStr: string): any;
    private static replacer;
    private static serialize;
    private static deserialize;
}
declare abstract class BehaviorConditional extends BehaviorTask {
    constructor(handle?: never);
}
interface IClientReceive {
    (msg: string): void;
}
interface IClientPayReceive {
    (msg: string, state: number): void;
}
interface IBehaviorHandler {
    (source: BehaviorTree): void;
}
declare class BehaviorTree extends Component {
    private _on_behavior_start;
    private _on_behavior_end;
    private _on_behavior_restart;
    constructor(handle: never);
    get startWhenEnabled(): boolean;
    get asynchronousLoad(): boolean;
    get pauseWhenDisabled(): boolean;
    get restartWhenComplete(): boolean;
    get logTaskChanges(): boolean;
    get resetValuesOnRestart(): boolean;
    get externalBehavior(): ExternalBehaviorTree;
    get behaviorName(): string;
    get behaviorDescription(): string;
    get executionStatus(): BehaviorTaskStatus;
    get updateInterval(): BehaviorUpdateInterval;
    get specifiedSecond(): number;
    get OnBehaviorStart(): IBehaviorHandler;
    get OnBehaviorRestart(): IBehaviorHandler;
    get OnBehaviorStop(): IBehaviorHandler;
    set startWhenEnabled(val: boolean);
    set asynchronousLoad(val: boolean);
    set pauseWhenDisabled(val: boolean);
    set restartWhenComplete(val: boolean);
    set logTaskChanges(val: boolean);
    set resetValuesOnRestart(val: boolean);
    set externalBehavior(external: ExternalBehaviorTree);
    set behaviorName(val: string);
    set behaviorDescription(val: string);
    set updateInterval(val: BehaviorUpdateInterval);
    set specifiedSecond(val: number);
    set OnBehaviorStart(handler: IBehaviorHandler);
    set OnBehaviorRestart(handler: IBehaviorHandler);
    set OnBehaviorStop(handler: IBehaviorHandler);
    GetBehaviorSource(): BehaviorSource;
    EnableBehavior(): void;
    DisableBehavior(): void;
    PauseBehavior(val: boolean): void;
    Tick(): void;
    FindTask<T extends BehaviorTask>(type: Traits_Constructor<T>): T;
    FindTasks<T extends BehaviorTask>(type: Traits_Constructor<T>): T[];
    FindTaskWithName(name: string): BehaviorTask;
    FindTasksWithName(name: string): BehaviorTask[];
    private static handle_check;
    private static get_start_when_enabled;
    private static get_asynchronous_load;
    private static get_pause_when_disabled;
    private static get_restart_when_complete;
    private static get_log_task_changes;
    private static get_reset_val_on_restart;
    private static get_ext_behavior;
    private static get_name;
    private static get_desc;
    private static get_status;
    private static get_update_interval;
    private static get_specified_second;
    private static set_start_when_enabled;
    private static set_asynchronous_load;
    private static set_pause_when_disabled;
    private static set_restart_when_complete;
    private static set_log_task_changes;
    private static set_reset_val_on_restart;
    private static set_ext_behavior;
    private static set_name;
    private static set_desc;
    private static set_on_behavior_start;
    private static set_on_behavior_restart;
    private static set_on_behavior_end;
    private static set_update_interval;
    private static set_specified_second;
    private static get_behavior_src;
    private static set_behavior_src;
    private static enable_behavior;
    private static disable_behavior;
    private static pause_behavior;
    private static tick;
    private static find_task;
    private static find_task_list;
    private static find_task_with_name;
    private static find_task_list_with_name;
}
type Traits_Constructor<T> = T extends Function ? Function : (Function & {
    prototype: T;
});
type Traits_PrimitiveType<T> = T extends String ? string : T extends Number ? number : T extends Boolean ? boolean : T;
declare class SpriteRenderer extends Renderer {
    constructor(handle: never);
    get sprite(): Sprite | null;
    get color(): Color;
    get sortingLayer(): number;
    get order(): number;
    set sprite(value: Sprite | null);
    set color(value: Color);
    set sortingLayer(value: number);
    set order(value: number);
    private static handle_check;
    private static get_sprite;
    private static get_color;
    private static get_sortingLayer;
    private static get_order;
    private static set_sprite;
    private static set_color;
    private static set_sortingLayer;
    private static set_order;
}
declare enum AudioDeviceState {
    NONE = 0,
    ADDED = 1,
    REMOVED = 2,
    DEFAULT_CHANGED = 3
}
interface IAudioDeviceChanged {
    (state: AudioDeviceState): void;
}
declare class AudioListener extends Component {
    private static _on_change_device;
    constructor(handle: never);
    static get deviceChangedHandler(): IAudioDeviceChanged;
    static get volume(): number;
    static get pause(): boolean;
    static set volume(val: number);
    static set pause(val: boolean);
    static set deviceChangedHandler(handler: IAudioDeviceChanged);
    private static handle_check;
    private static get_volume_impl;
    private static get_pause_impl;
    private static set_volume_impl;
    private static set_pause_impl;
    private static set_dev_changed_callback_impl;
}
declare abstract class AnimationCurve extends EngineObject {
    private _handle;
    get curveType(): AnimationCurveType;
    SmoothTangents(index: number, weight: number): void;
    private static smooth_tangents_impl;
    private static curve_type_impl;
}
interface IOnAudioRecordCallBack {
    (type: number, time: number, path: string): void;
}
interface IUploadVoiceFileResult {
    (code: number, url: string): void;
}
declare class Rigidbody extends Component {
    constructor(handle: never);
    get isSleeping(): boolean;
    get useGravity(): boolean;
    get isKinematic(): boolean;
    get freezeRotation(): boolean;
    get detectCollisions(): boolean;
    get solverVelocityIterations(): number;
    get solverIterations(): number;
    get mass(): number;
    get drag(): number;
    get angularDrag(): number;
    get sleepThreshold(): number;
    get maxAngularVelocity(): number;
    get maxLinearVelocity(): number;
    get maxDepenetrationVelocity(): number;
    get position(): Vector3;
    get velocity(): Vector3;
    get angularVelocity(): Vector3;
    get centerOfMass(): Vector3;
    get worldCenterOfMass(): Vector3;
    get inertiaTensor(): Vector3;
    get rotation(): Quaternion;
    get inertiaTensorRotation(): Quaternion;
    get constraints(): RigidbodyConstraints;
    get interpolation(): RigidbodyInterpolation;
    get collisionDetectionMode(): CollisionDetectionMode;
    set solverVelocityIterations(value: number);
    set maxAngularVelocity(value: number);
    set maxLinearVelocity(value: number);
    set sleepThreshold(value: number);
    set solverIterations(value: number);
    set interpolation(value: RigidbodyInterpolation);
    set velocity(value: Vector3);
    set angularVelocity(value: Vector3);
    set drag(value: number);
    set angularDrag(value: number);
    set mass(value: number);
    set useGravity(value: boolean);
    set maxDepenetrationVelocity(value: number);
    set isKinematic(value: boolean);
    set isSimulate(value: boolean);
    set freezeRotation(value: boolean);
    set constraints(value: RigidbodyConstraints);
    set collisionDetectionMode(value: CollisionDetectionMode);
    set centerOfMass(value: Vector3);
    set inertiaTensorRotation(value: Quaternion);
    set inertiaTensor(value: Vector3);
    set detectCollisions(value: boolean);
    set position(value: Vector3);
    set rotation(value: Quaternion);
    set density(value: number);
    AddExplosionForce(explosionForce: number, explosionPosition: Vector3, explosionRadius: number, upwardsModifier: number, mode: ForceMode): void;
    AddForce(force: Vector3, mode: ForceMode): void;
    AddForceAtPosition(force: Vector3, position: Vector3, mode: ForceMode): void;
    AddRelativeForce(force: Vector3, mode: ForceMode): void;
    AddRelativeTorque(torque: Vector3, mode: ForceMode): void;
    AddTorque(torque: Vector3, mode: ForceMode): void;
    ClosestPointOnBounds(position: Vector3): ClosestPoint;
    GetPointVelocity(value: Vector3): Vector3;
    GetRelativePointVelocity(value: Vector3): Vector3;
    MovePosition(value: Vector3): void;
    MoveRotation(value: Quaternion): void;
    ResetCenterOfMass(): void;
    ResetInertiaTensor(): void;
    Sleep(): void;
    WakeUp(): void;
    SweepTest(direction: Vector3, maxDistance?: number): RaycastHit | undefined;
    private static handle_check;
    private static getIsSleeping_impl;
    private static getUseGravity_impl;
    private static getIsKinematic_impl;
    private static getFreezeRotation_impl;
    private static getDetectCollisions_impl;
    private static getSolverVelocityIterations_impl;
    private static getSolverIterations_impl;
    private static getMass_impl;
    private static getDrag_impl;
    private static getAngularDrag_impl;
    private static getSleepThreshold_impl;
    private static getMaxAngularVelocity_impl;
    private static getMaxLinearVelocity_impl;
    private static getMaxDepenetrationVelocity_impl;
    private static getPosition_impl;
    private static getVelocity_impl;
    private static getAngularVelocity_impl;
    private static getCenterOfMass_impl;
    private static getWorldCenterOfMass_impl;
    private static getInertiaTensor_impl;
    private static getRotation_impl;
    private static getInertiaTensorRotation_impl;
    private static getConstraints_impl;
    private static getInterpolation_impl;
    private static getCollisionDetectionMode_impl;
    private static setSolverVelocityIterations_impl;
    private static setMaxAngularVelocity_impl;
    private static setMaxLinearVelocity_impl;
    private static setSleepThreshold_impl;
    private static setSolverIterations_impl;
    private static setInterpolation_impl;
    private static setVelocity_impl;
    private static setAngularVelocity_impl;
    private static setDrag_impl;
    private static setAngularDrag_impl;
    private static setMass_impl;
    private static setUseGravity_impl;
    private static setMaxDepenetrationVelocity_impl;
    private static setIsKinematic_impl;
    private static setIsSimulate_impl;
    private static setFreezeRotation_impl;
    private static setConstraints_impl;
    private static setCollisionDetectionMode_impl;
    private static setCenterOfMass_impl;
    private static setInertiaTensorRotation_impl;
    private static setInertiaTensor_impl;
    private static setDetectCollisions_impl;
    private static setPosition_impl;
    private static setRotation_impl;
    private static AddExplosionForce_impl;
    private static AddForce_impl;
    private static AddForceAtPosition_impl;
    private static AddRelativeForce_impl;
    private static AddRelativeTorque_impl;
    private static AddTorque_impl;
    private static ClosestPointOnBounds_impl;
    private static GetPointVelocity_impl;
    private static GetRelativePointVelocity_impl;
    private static MovePosition_impl;
    private static MoveRotation_impl;
    private static ResetCenterOfMass_impl;
    private static ResetInertiaTensor_impl;
    private static SetDensity_impl;
    private static Sleep_impl;
    private static WakeUp_impl;
    private static SweepTest_impl;
}
declare class AnimationPlaySetting {
    private _wrap_mode;
    private _speed;
    private _trigger_events;
    private _apply_root;
    get wrapMode(): WrapMode;
    get speed(): number;
    get triggerEvents(): boolean;
    get applyRoot(): boolean;
    set wrapMode(mode: WrapMode);
    set speed(val: number);
    set triggerEvents(val: boolean);
    set applyRoot(val: boolean);
}
declare class Image extends Control {
    constructor(handle: never);
    get imageUV(): Rect;
    get material(): Material | null;
    get texture(): Texture | null;
    get sprite(): Sprite | null;
    get spriteSequence(): SpriteSequence | null;
    get imageRegion(): Rect;
    get maskable(): boolean;
    get raycastTarget(): boolean;
    get preserveAspect(): boolean;
    get fillCenter(): boolean;
    get fillClockwise(): boolean;
    get fillAmount(): number;
    get materialName(): string;
    get raycastPadding(): Rect;
    get color(): Color;
    get imageType(): ImageType;
    get fillMethod(): FillMethod;
    get fillOrigin(): FillOrigin;
    get fillStartPoint(): FillStartPoint;
    get loopCount(): number;
    get loop(): boolean;
    get isPlaying(): boolean;
    get isStop(): boolean;
    get isPause(): boolean;
    set material(material: Material);
    set texture(texture: Texture);
    set sprite(sprite: Sprite);
    set spriteSequence(sprite_sequence: SpriteSequence | null);
    set imageRegion(region: Rect);
    set maskable(enable: boolean);
    set raycastTarget(enable: boolean);
    set preserveAspect(enable: boolean);
    set fillCenter(enable: boolean);
    set fillClockwise(enable: boolean);
    set fillAmount(amount: number);
    set raycastPadding(padding: Rect);
    set color(color: Color);
    set imageType(type: ImageType);
    set fillMethod(type: FillMethod);
    set fillOrigin(type: FillOrigin);
    set fillStartPoint(type: FillStartPoint);
    set loopCount(value: number);
    set loop(value: boolean);
    GetTextureUUID(): string;
    GetAtlasUUID(): string;
    SetAtlasTexture(atlasUUID: string, textureUUID: string): void;
    SetNativeSize(): void;
    Reset(): void;
    Play(): void;
    Stop(): void;
    Continue(): void;
    Pause(): void;
    GoToSprite(value: number): void;
    PreviousSprite(): void;
    NextSprite(): void;
    AddEvent(event: ImageEvent | ControlBaseEvent | ControlEvent, callBack: IControlEventCallBack | IControlEventCallBackI | IControlEventCallBackS): void;
    ClearEvent(event: ImageEvent | ControlBaseEvent | ControlEvent, callBack?: IControlEventCallBack | IControlEventCallBackI | IControlEventCallBackS | null): void;
    private static handle_check;
    private static get_image_uv;
    private static get_material;
    private static get_texture;
    private static get_sprite;
    private static get_sprite_sequence;
    private static get_image_region;
    private static get_maskable;
    private static get_raycast_target;
    private static get_preserve_aspect;
    private static get_fill_center;
    private static get_fill_clockwise;
    private static get_fill_amount;
    private static get_material_name;
    private static get_texture_uuid;
    private static get_atlas_uuid;
    private static get_raycast_padding;
    private static get_color;
    private static get_image_type;
    private static get_fill_method;
    private static get_fill_origin;
    private static get_fill_start_point;
    private static set_material;
    private static set_texture;
    private static set_atlas_texture;
    private static set_sprite;
    private static set_sprite_sequence;
    private static set_image_region;
    private static set_maskable;
    private static set_raycast_target;
    private static set_preserve_aspect;
    private static set_fill_center;
    private static set_fill_clockwise;
    private static set_fill_amount;
    private static set_raycast_padding;
    private static set_color;
    private static set_image_type;
    private static set_fill_method;
    private static set_fill_origin;
    private static set_fill_start_point;
    private static set_native_size;
    private static get_loop_count;
    private static is_loop;
    private static is_playing;
    private static is_stop;
    private static is_pause;
    private static set_loop_count;
    private static set_loop;
    private static reset_impl;
    private static play_impl;
    private static stop_impl;
    private static continue_impl;
    private static pause_impl;
    private static end_impl;
    private static go_to_sprite_impl;
    private static previous_sprite_impl;
    private static next_sprite_impl;
}
declare class BehaviorTaskGuard extends BehaviorDecorator {
    constructor(handle?: never);
    get maxTaskAccessCount(): BehaviorInt;
    get linkedTaskGuards(): BehaviorTaskGuard[];
    get waitUntilTaskAvailable(): BehaviorBool;
    set maxTaskAccessCount(val: BehaviorInt);
    set linkedTaskGuards(list: BehaviorTaskGuard[]);
    set waitUntilTaskAvailable(val: BehaviorBool);
    SetMaxTaskAccessCount(count: number): void;
    SetWaitUntilTaskAvailable(val: boolean): void;
    private static get_max_access_cnt;
    private static get_linked_task_guards;
    private static get_wait_until_available;
    private static set_max_access_cnt;
    private static set_linked_task_guards;
    private static set_wait_until_available;
    private static set_max_access_cnt_var;
    private static set_wait_until_available_var;
}
declare class BehaviorUntilFailure extends BehaviorDecorator {
    constructor(handle?: never);
}
declare class Queue<T> {
    private _items;
    private _type;
    constructor(type: Traits_Constructor<T>);
    get(index: number): Traits_PrimitiveType<T>;
    get items(): Traits_PrimitiveType<T>[];
    get count(): number;
    Enqueue(item: Traits_PrimitiveType<T>): void;
    Dequeue(): Traits_PrimitiveType<T>;
    Peek(): Traits_PrimitiveType<T>;
    Clear(): void;
    Swap(rhs: Queue<T>): void;
    SerializeJson(obj: any): void;
    DeserializeJson(obj: any): void;
    toString(): string;
    forEach(callback: (item: Traits_PrimitiveType<T>) => void): void;
    [Symbol.iterator](): Generator<Traits_PrimitiveType<T>, void, unknown>;
}
declare class LimbIK extends IKSolver {
    constructor(handle: never);
    get target(): Transform;
    get bone1(): Transform;
    get bone2(): Transform;
    get bone3(): Transform;
    get bendGoal(): Transform;
    get maintainRotationWeight(): number;
    get bendModifierWeight(): number;
    get bendModifier(): BendModifier;
    set target(transform: Transform);
    set bone1(transform: Transform);
    set bone2(transform: Transform);
    set bone3(transform: Transform);
    set bendGoal(transform: Transform);
    set maintainRotationWeight(MRweight: number);
    set bendModifierWeight(BMweight: number);
    set bendModifier(bend: BendModifier);
    private static handle_check;
    private static getTarget;
    private static getBone1;
    private static getBone2;
    private static getBone3;
    private static getBendGoal;
    private static getMaintainRotationWeight;
    private static getBendModifierWeight;
    private static getBendModifier;
    private static setTarget;
    private static setBone1;
    private static setBone2;
    private static setBone3;
    private static setBendGoal;
    private static setMaintainRotationWeight;
    private static setBendModifierWeight;
    private static setBendModifier;
}
declare class ScrollView extends Control {
    private _constructorList;
    private _createCb;
    private _renderCb;
    private _recycleCb;
    private _controlItemMap;
    constructor(handle: never);
    get bg(): Control;
    get hScrollbar(): Control;
    get vScrollbar(): Control;
    get content(): Control;
    get horizontal(): boolean;
    get vertical(): boolean;
    get inertia(): boolean;
    get elasticity(): number;
    get decelerationRate(): number;
    get scrollSensitivity(): number;
    get movementType(): MovementType;
    get hScrollbarVisibility(): VisibilityType;
    get vScrollbarVisibility(): VisibilityType;
    get virtualItemCount(): number;
    set horizontal(enable: boolean);
    set vertical(enable: boolean);
    set inertia(enable: boolean);
    set elasticity(elasticity: number);
    set decelerationRate(rate: number);
    set scrollSensitivity(sensitivity: number);
    set movementType(type: MovementType);
    set hScrollbarVisibility(mode: VisibilityType);
    set vScrollbarVisibility(mode: VisibilityType);
    Close(): void;
    ScrollTo(value: Vector2, time?: number): void;
    InitVirtual(constructors: Iterable<new () => ControlItem>, createCb: IVirtualItemCreateCallBack | null, renderCb: IVirtualItemRenderCallBack | null, recycleCb: IVirtualItemRecycleCallBack | null, capacitys?: number[]): void;
    AddVirtualItem(index: number, id?: number, jsonStr?: string): void;
    ClearVirtualItem(): void;
    RemoveVirtualItem(id: number): void;
    BuildVirtual(): void;
    GetVirtualItem(index: number): LayoutItem;
    AddEvent(event: ScrollViewEvent | ControlBaseEvent | ControlEvent, callBack: IControlEventCallBack | IControlEventCallBackI | IControlEventCallBackS): void;
    ClearEvent(event: ScrollViewEvent | ControlBaseEvent | ControlEvent, callBack?: IControlEventCallBack | IControlEventCallBackI | IControlEventCallBackS | null): void;
    private CreateCb;
    private RenderCb;
    private RecycleCb;
    private static handle_check;
    private static get_bg_control;
    private static get_h_scrollbar_control;
    private static get_v_scrollbar_control;
    private static get_content_control;
    private static get_horizontal;
    private static get_vertical;
    private static get_inertia;
    private static get_elasticity;
    private static get_deceleration_rate;
    private static get_scroll_sensitivity;
    private static get_movement_type;
    private static get_h_scrollbar_visibility;
    private static get_v_scrollbar_visibility;
    private static set_horizontal;
    private static set_vertical;
    private static set_inertia;
    private static set_elasticity;
    private static set_deceleration_rate;
    private static set_scroll_sensitivity;
    private static set_movement_type;
    private static set_h_scrollbar_visibility;
    private static set_v_scrollbar_visibility;
    private static scroll_to;
    private static init_virtual;
    private static add_virtual_item;
    private static clear_virtual_item;
    private static remove_virtual_item;
    private static build_virtual;
    private static get_virtual_item_count;
    private static get_virtual_item;
}
declare class NetServerSender implements IContracter {
    FindEntity(entityID: string): NetEntity;
    CreateNewEntity(prefab_id: string, userdata: string, callback?: (entity: NetEntity) => void): void;
    CurrentConversation(): NetConversation;
}
declare class ObjectHashSet<T extends (IComparable & IHashCodeProvider)> {
    private _count;
    private _null_added;
    private _handle;
    get count(): number;
    Add(item: T | null): void;
    Remove(item: T | null): void;
    Contains(item: T | null): boolean;
    Clear(): void;
    toString(): string;
    forEach(callback: (item: T) => void): void;
    [Symbol.iterator](): Generator<T, void, unknown>;
}
declare class Convert {
    static ToBoolean(val: any): boolean;
    static ToString(val: any): string;
    static ToNumber(val: any): number;
}
declare class BehaviorCooldown extends BehaviorDecorator {
    constructor(handle?: never);
    get duration(): BehaviorFloat;
    set duration(val: BehaviorFloat);
    SetDuration(duration: number): void;
    private static get_duration;
    private static set_duration;
    private static set_duration_var;
}
declare class AnimationState extends EngineObject {
    private _handle;
    constructor(handle: never);
    get blendMode(): AnimationBlendMode;
    get clip(): AnimationClip | null;
    get enable(): boolean;
    get isActivated(): boolean;
    get layer(): number;
    get name(): string;
    get normalizedSpeed(): number;
    get normalizedTime(): number;
    get length(): number;
    get speed(): number;
    get speedSync(): number;
    get time(): number;
    get weight(): number;
    get wrapMode(): WrapMode;
    set blendMode(val: AnimationBlendMode);
    set enable(val: boolean);
    set layer(val: number);
    set name(val: string);
    set normalizedSpeed(val: number);
    set normalizedSpeedSync(val: number);
    set normalizedTime(val: number);
    set speed(val: number);
    set time(val: number);
    set weight(val: number);
    set wrapMode(mode: WrapMode);
    Stop(): void;
    private static handle_check;
    private static get_blend_mode_impl;
    private static get_clip_impl;
    private static get_enable_impl;
    private static get_is_activated_impl;
    private static get_layer_impl;
    private static get_name_impl;
    private static get_normalized_speed_impl;
    private static get_normalized_time_impl;
    private static get_length_impl;
    private static get_parent_impl;
    private static get_speed_impl;
    private static get_speed_sync_impl;
    private static get_time_impl;
    private static get_weight_impl;
    private static get_wrap_mode_impl;
    private static set_blend_mode_impl;
    private static set_enable_impl;
    private static set_layer_impl;
    private static set_name_impl;
    private static set_normalize_speed_impl;
    private static set_normalized_speed_sync_impl;
    private static set_normalized_time_impl;
    private static set_speed_impl;
    private static set_time_impl;
    private static set_trigger_events_impl;
    private static set_weight_impl;
    private static set_wrap_mode_impl;
    private static stop_impl;
}
declare class ControlItem {
    private _control;
    private _id;
    constructor();
    get control(): Control;
    private set control(value);
    private set id(value);
    AddEvent<T extends ControlBaseEvent | ButtonEvent | DropdownEvent | GUIMaskEvent | GUISpriteSequenceEvent | ImageEvent | ScrollbarEvent | ScrollViewEvent | SliderEvent | TextEvent | TextBoxEvent | ToggleEvent | ControlEvent>(control: Control, event: T, callback: (id: number, control: Control, arg: number | String | null) => void): void;
    ClearEvent<T extends ControlBaseEvent | ButtonEvent | DropdownEvent | GUIMaskEvent | GUISpriteSequenceEvent | ImageEvent | ScrollbarEvent | ScrollViewEvent | SliderEvent | TextEvent | TextBoxEvent | ToggleEvent | ControlEvent>(control: Control, event: T): void;
}
declare class AnimatorStateInfo {
    private _name;
    private _full_name;
    private _is_valid;
    private _speed;
    private _state_time;
    private _normalized_state_time;
    private _loop;
    private _animator_length;
    get name(): string;
    get isValid(): boolean;
    get speed(): number;
    get stateTime(): number;
    get normalizedStateTime(): number;
    get loop(): boolean;
    get animatorLength(): number;
    IsName(name: string): boolean;
    private static CreateFromInternalBuffer;
}
declare class FloatCurve extends AnimationCurve {
    constructor(handle?: never);
    Evaluate(value: number): number;
    private static handle_check;
    private static alloc_object;
    private static evaluate_impl;
}
declare class ObjectDictionaryBucket<K extends (IComparable & IHashCodeProvider), V> {
    hash: number;
    items: DictionaryItem<K, V>[];
    constructor(hash: number);
    Get(key: Traits_PrimitiveType<K>): any | undefined;
    Set(key: Traits_PrimitiveType<K>, value: any, overridable: boolean): boolean;
    Remove(key: Traits_PrimitiveType<K>): boolean;
    Contains(key: Traits_PrimitiveType<K>): boolean;
}
declare class UUID128 {
    static NewUUID(): String;
    private static new_uuid_impl;
}
declare class BoneWeight {
    weight0: number;
    weight1: number;
    weight2: number;
    weight3: number;
    index0: number;
    index1: number;
    index2: number;
    index3: number;
    Normalize(): void;
}
declare class MeshCollider extends Collider {
    constructor(handle: never);
    get convex(): boolean;
    get sharedMesh(): Mesh | null;
    set convex(value: boolean);
    set sharedMesh(mesh: Mesh);
    set isTrigger(value: boolean);
    private static handle_check;
    private static getConvex;
    private static getSharedMesh;
    private static setConvex;
    private static setSharedMesh;
    private static setIsTrigger_Mesh;
}
declare class EditorComponentSettings {
    static DecorateName(name: string): (target: any, proptyName: string) => void;
    static Header(name: string): (target: any, proptyName: string) => void;
    static Tooltip(name: string): (target: any, proptyName: string) => void;
    static Range(min: number, max: number): (target: any, proptyName: string) => void;
    static CallEditor(path: string): (target: any, proptyName: string) => void;
}
declare class RectTransform extends EngineObject {
    private _handle;
    constructor(handle: never);
    get control(): Control;
    get name(): string;
    get anchorMax(): Vector2;
    get anchorMin(): Vector2;
    get offsetMin(): Vector2;
    get offsetMax(): Vector2;
    get rect(): Rect;
    get posX(): number;
    get posY(): number;
    get worldPosX(): number;
    get worldPosY(): number;
    get width(): number;
    get height(): number;
    get localPosition(): Vector2;
    get left(): number;
    get top(): number;
    get right(): number;
    get bottom(): number;
    get aabb(): Rect;
    get pivot(): Vector2;
    get localScale(): Vector2;
    get localRotation(): number;
    get worldScale(): Vector2;
    get worldRotation(): number;
    get worldPosition(): Vector2;
    get worldMatrix(): Matrix4x4;
    get siblingIndex(): number;
    set anchorMaxX(value: number);
    set anchorMaxY(value: number);
    set anchorMax(value: Vector2);
    set anchorMinX(value: number);
    set anchorMinY(value: number);
    set anchorMin(value: Vector2);
    set pivot(value: Vector2);
    set rect(value: Rect);
    set posX(value: number);
    set posY(value: number);
    set worldPosX(value: number);
    set worldPosY(value: number);
    set width(value: number);
    set height(value: number);
    set left(value: number);
    set top(value: number);
    set right(value: number);
    set bottom(value: number);
    set localScale(value: Vector2);
    set localRotation(value: number);
    set localPosition(value: Vector2);
    set worldPosition(value: Vector2);
    set siblingIndex(value: number);
    IsContain(value: Vector2): boolean;
    SetAsFirstSibling(): void;
    SetAsLastSibling(): void;
    toString(): string;
    private static handle_check;
    private static get_control;
    private static get_name;
    private static get_anchor_max;
    private static get_anchor_min;
    private static get_offset_min;
    private static get_offset_max;
    private static get_rect;
    private static get_pos_x;
    private static get_pos_y;
    private static get_world_pos_x;
    private static get_world_pos_y;
    private static get_width;
    private static get_height;
    private static get_local_position;
    private static get_left;
    private static get_top;
    private static get_right;
    private static get_bottom;
    private static get_aabb;
    private static get_pivot;
    private static get_local_scale;
    private static get_local_rotation;
    private static get_world_scale;
    private static get_world_rotation;
    private static get_world_position;
    private static get_world_matrix;
    private static get_sibling_index;
    private static set_anchor_max_x;
    private static set_anchor_max_y;
    private static set_anchor_max;
    private static set_anchor_min_x;
    private static set_anchor_min_y;
    private static set_anchor_min;
    private static set_rect;
    private static set_pos_x;
    private static set_pos_y;
    private static set_world_pos_x;
    private static set_world_pos_y;
    private static set_width;
    private static set_height;
    private static set_local_position;
    private static set_left;
    private static set_top;
    private static set_right;
    private static set_bottom;
    private static set_pivot;
    private static set_local_scale;
    private static set_local_rotation;
    private static set_world_position;
    private static set_sibling_index;
    private static is_contain;
    private static set_as_first_sibling;
    private static set_as_last_sibling;
}
declare class GameObject extends EngineObject {
    private _handle;
    private _transform;
    constructor(name?: string, scene?: Scene, handle?: never);
    get instanceID(): number;
    get transform(): Transform;
    get name(): string;
    get scene(): Scene;
    get enable(): boolean;
    get allowEnable(): boolean;
    get layer(): number;
    get isStatic(): boolean;
    get needDestroyed(): boolean;
    set name(name: string);
    set layer(new_layer: number);
    set enable(enable: boolean);
    set isStatic(is_static: boolean);
    AddComponent<T extends Component>(type: Traits_Constructor<T>): T | null;
    GetComponent<T extends Component>(type: Traits_Constructor<T>): T | null;
    GetComponentInChildren<T extends Component>(type: Traits_Constructor<T>): T | null;
    GetComponentInParent<T extends Component>(type: Traits_Constructor<T>): T | null;
    GetComponents<T extends Component>(type: Traits_Constructor<T>): T[] | null;
    GetComponentsInChildren<T extends Component>(type: Traits_Constructor<T>): T[] | null;
    GetComponentsInParent<T extends Component>(type: Traits_Constructor<T>): T[] | null;
    static DestroyGameObject(go: GameObject): void;
    static DestroyComponent(com: Component): void;
    static CreatePrimitive(type: PrimitiveType): GameObject;
    setOnBeforeDestroy(callback: () => void): void;
    static Instantiate(origin: GameObject, position?: Vector3, rotation?: Quaternion, parent?: Transform): GameObject;
    private static alloc_object;
    private static handle_check;
    private static get_inst_id_impl;
    private static get_transform;
    private static get_name_impl;
    private static get_scene_impl;
    private static get_enable_impl;
    private static get_allow_enable_impl;
    private static get_layer_impl;
    private static get_is_static_impl;
    private static get_is_need_destroyed_impl;
    private static set_name_impl;
    private static set_layer_impl;
    private static set_enable_impl;
    private static set_is_static_impl;
    private static add_com_impl;
    private static get_com_impl;
    private static get_com_child_impl;
    private static get_com_parent_impl;
    private static get_coms_impl;
    private static get_coms_child_impl;
    private static get_coms_parent_impl;
    private static destroy_go_impl;
    private static destroy_com_impl;
    private static create_primitive_impl;
    private static set_on_before_destroy;
    private static inst_impl;
}
declare class PSGradient {
    private _handle;
    constructor(handle?: never);
    get gradientMode(): PSGradientMode;
    get maxColor(): Color;
    get maxGradient(): Gradient | null;
    get minColor(): Color;
    get minGradient(): Gradient | null;
    get isOptimized(): boolean;
    set gradientMode(value: PSGradientMode);
    set maxColor(value: Color);
    set minColor(value: Color);
    Evaluate(time: number, random_value?: number): Color;
    BuildGradients(): boolean;
    CopyTo(target: PSGradient): void;
    Reset(mode?: PSGradientMode, color?: Color, begin_color?: Color, end_color?: Color): void;
    private static alloc_object;
    private static handle_check;
    private static get_gradient_mode;
    private static get_max_color;
    private static get_max_gradient;
    private static get_min_color;
    private static get_min_gradient;
    private static get_is_optimized;
    private static set_gradient_mode;
    private static set_max_color;
    private static set_min_color;
    private static evaluate;
    private static build_gradients;
    private static copy_to;
    private static reset;
}
declare class ScenePartitionViewer extends Component {
    constructor(handle: never);
    private static handle_check;
}
declare class Directory {
    static Exists(path: string): boolean;
    static Create(path: string): boolean;
    static IsEmpty(path: string): boolean;
    static GetParent(path: string): string;
    static GetFiles(path: string): string[];
    static GetDirectories(path: string): string[];
    static Delete(path: string): boolean;
    static Copy(src: string, dst: string): boolean;
    private static Exists_impl;
    private static Create_impl;
    private static IsEmpty_impl;
    private static GetParent_impl;
    private static GetFiles_impl;
    private static GetDirectories_impl;
    private static Delete_impl;
    private static Copy_impl;
}
declare class SystemInfo {
    static get needConvertGLSLFunction(): boolean;
    static get totalMemorySize(): number;
    static get availMemorySize(): number;
    static get clientInfo(): string;
    private static IsNeedConvertGLSLFunction;
    private static GetTotalMemorySize;
    private static GetAvailMemorySize;
    private static GetClientInfo;
}
declare class BehaviorSource {
    private _handle;
    constructor(handle: never);
    get EntryTask(): BehaviorEntry;
    get RootTask(): BehaviorTask;
    get owner(): BehaviorTree;
    set EntryTask(task: BehaviorEntry);
    set RootTask(task: BehaviorTask);
    AddVariable<T extends BehaviorVariable>(name: string, type: Traits_Constructor<T>): T | null;
    GetVariable(name: string): BehaviorVariable;
    GetAllVariables(): BehaviorVariable[];
    FindTask<T extends BehaviorTask>(type: Traits_Constructor<T>): T;
    FindTasks<T extends BehaviorTask>(type: Traits_Constructor<T>): T[];
    FindTaskWithName(name: string): BehaviorTask;
    FindTasksWithName(name: string): BehaviorTask[];
    private static handle_check;
    private static get_entry;
    private static get_root;
    private static add_variable;
    private static get_variable;
    private static get_all_variable;
    private static set_entry;
    private static set_root;
    private static create_task;
    private static get_owner;
    private static find_task;
    private static find_task_list;
    private static find_task_with_name;
    private static find_task_list_with_name;
}
declare class Animator extends Component {
    constructor(handle: never);
    get animatorData(): AnimatorData;
    get speed(): number;
    get cullMode(): AnimationCullMode;
    set animatorData(value: AnimatorData);
    set speed(value: number);
    set cullMode(mode: AnimationCullMode);
    getLayerWeight(layer_index: number): number;
    setLayerWeight(layer_index: number, weight: number): void;
    CrossFade(name: string, normalized_transition_duration: number, layer_index?: number, normalized_time_offset?: number, normalized_transition_time?: number): void;
    CrossFadeInFixedTime(name: string, normalized_transition_duration: number, layer_index?: number, fixed_time_offset?: number, fixed_transition_time?: number): void;
    Play(name: string, layer_index?: number, normalized_time_offset?: number): void;
    PlayInFixedTime(name: string, layer_index?: number, fixed_time_offset?: number): void;
    GetCurrentStateInfo(layer_index: number): AnimatorStateInfo;
    GetStateInfo(layer_index: number, name: string): AnimatorStateInfo;
    GetNextStateInfo(layer_index: number): AnimatorStateInfo;
    GetBool(name: string): boolean | undefined;
    GetFloat(name: string): number | undefined;
    GetInteger(name: string): number | undefined;
    SetBool(name: string, value: boolean): boolean | undefined;
    SetFloat(name: string, value: number): boolean | undefined;
    SetInteger(name: string, value: number): boolean | undefined;
    SetTrigger(name: string): boolean | undefined;
    ResetTrigger(name: string): boolean | undefined;
    private static handle_check;
    private static get_data_impl;
    private static get_speed_impl;
    private static get_cull_mode_impl;
    private static get_layer_weight_impl;
    private static set_data_impl;
    private static set_speed_impl;
    private static set_cull_mode_impl;
    private static set_layer_weight_impl;
    private static cross_fade_impl;
    private static cross_fade_ftime_impl;
    private static play_impl;
    private static play_fixed_impl;
    private static get_current_state_impl;
    private static get_state_impl;
    private static get_next_state_impl;
    private static get_bool_impl;
    private static get_float_impl;
    private static get_integer_impl;
    private static set_bool_impl;
    private static set_float_impl;
    private static set_integer_impl;
    private static set_trigger_impl;
    private static reset_trigger;
}
declare class TrailRenderer extends Renderer {
    constructor(handle: never);
    get alignment(): LineAlignment;
    get autodestruct(): boolean;
    get colorGradient(): Gradient;
    get emitting(): boolean;
    get endColor(): Color;
    get endWidth(): number;
    get generateLightingData(): boolean;
    get minVertexDistance(): number;
    get numCapVertices(): number;
    get numCornerVertices(): number;
    get positionCount(): number;
    get startColor(): Color;
    get startWidth(): number;
    get textrueMode(): LineTextureMode;
    get time(): number;
    get widthCurve(): Curve;
    get widthMultiplier(): number;
    set alignment(align: LineAlignment);
    set autodestruct(auto: boolean);
    set colorGradient(color: Gradient);
    set emitting(emit: boolean);
    set endColor(color: Color);
    set endWidth(width: number);
    set generateLightingData(gen: boolean);
    set minVertexDistance(num: number);
    set numCapVertices(num: number);
    set numCornerVertices(num: number);
    set positionCount(count: number);
    set startColor(color: Color);
    set startWidth(width: number);
    set textrueMode(mode: LineTextureMode);
    set time(num: number);
    set widthCurve(width_curve: Curve);
    set widthMultiplier(mul: number);
    AddPosition(pos: Vector3): void;
    AddPositions(pos: Vector3[]): void;
    Clear(): void;
    GetPosition(index: number): Vector3;
    GetPositions(): Vector3[];
    SetPosition(index: number, pos: Vector3): void;
    SetPositions(pos: Vector3[]): void;
    private static handle_check;
    private static get_alignment_impl;
    private static get_autodestruct_impl;
    private static get_color_gradient_impl;
    private static get_emitting_impl;
    private static get_gen_light_data_impl;
    private static get_min_vertex_distance_impl;
    private static get_num_cap_vertices_impl;
    private static get_num_corner_vertices_impl;
    private static get_position_count_impl;
    private static get_texture_mode_impl;
    private static get_time_impl;
    private static get_width_curve_impl;
    private static get_width_multiplier_impl;
    private static set_alignment_impl;
    private static set_autodestruct_impl;
    private static set_color_gradient_impl;
    private static set_emitting_impl;
    private static set_gen_light_data_impl;
    private static set_min_vertex_distance_impl;
    private static set_num_cap_vertices_impl;
    private static set_num_corner_vertices_impl;
    private static set_position_count_impl;
    private static set_texture_mode_impl;
    private static set_time_impl;
    private static set_width_curve_impl;
    private static set_width_multiplier_impl;
    private static add_position_impl;
    private static add_positions_impl;
    private static clear_impl;
    private static get_position_impl;
    private static get_positions_impl;
    private static set_position_impl;
    private static set_positions_impl;
}
declare class Cubemap extends Texture {
    constructor(handle: never);
    private static handle_check;
}
declare class Dictionary<K extends (IComparable & IHashCodeProvider) | Number | String | Boolean, V> {
    private _handle;
    private _key_type;
    private _value_type;
    constructor(key_type: Traits_Constructor<K>, value_type: Traits_Constructor<V>);
    get count(): number;
    Get(key: Traits_PrimitiveType<K>): Traits_PrimitiveType<V>;
    Set(key: Traits_PrimitiveType<K>, value: Traits_PrimitiveType<V>): void;
    Add(key: Traits_PrimitiveType<K>, value: Traits_PrimitiveType<V>): void;
    Remove(key: Traits_PrimitiveType<K>): void;
    Contains(key: Traits_PrimitiveType<K>): boolean;
    Clear(): void;
    SerializeJson(obj: any): void;
    DeserializeJson(obj: any): void;
    toString(): string;
    forEach(callback: (key: Traits_PrimitiveType<K>, value: Traits_PrimitiveType<V>) => void): void;
    [Symbol.iterator](): Generator<DictionaryItem<K, V>, void, unknown>;
}
declare class FastAA extends PostEffectBase {
    constructor(handle?: never);
    get constrastThreshold(): number;
    get relativeThreshold(): number;
    set constrastThreshold(val: number);
    set relativeThreshold(val: number);
    private static alloc_object_impl;
    private static handle_check;
    private static get_constrastThreshold_impl;
    private static get_relativeThreshold_impl;
    private static set_constrastThreshold_impl;
    private static set_relativeThreshold_impl;
}
declare class Font extends AssetObject {
    private constructor();
    private static handle_check;
}
declare class HashSet<T extends (IComparable & IHashCodeProvider) | Number | String | Boolean> {
    private _handle;
    private _type;
    constructor(type: Traits_Constructor<T>);
    get count(): number;
    Add(item: Traits_PrimitiveType<T>): void;
    Remove(item: Traits_PrimitiveType<T>): void;
    Contains(item: Traits_PrimitiveType<T>): boolean;
    Clear(): void;
    Swap(rhs: HashSet<T>): void;
    SerializeJson(obj: any): void;
    DeserializeJson(obj: any): void;
    toString(): string;
    forEach(callback: (item: Traits_PrimitiveType<T>) => void): void;
    [Symbol.iterator](): Generator<any, void, unknown>;
}
declare class Dropdown extends TransitionControl {
    constructor(handle: never);
    get bg(): Control;
    get label(): Control;
    get arrow(): Control;
    get view(): Control;
    get value(): number;
    get optionsCount(): number;
    set value(value: number);
    ClearAllOptions(): void;
    RemoveOptions(value: number): void;
    AddOptions(value: string): void;
    GetOptions(value: number): string;
    AddEvent(event: DropdownEvent | ControlBaseEvent | ControlEvent, callBack: IControlEventCallBack | IControlEventCallBackI | IControlEventCallBackS): void;
    ClearEvent(event: DropdownEvent | ControlBaseEvent | ControlEvent, callBack?: IControlEventCallBack | IControlEventCallBackI | IControlEventCallBackS | null): void;
    private static handle_check;
    private static get_bg_control;
    private static get_label_control;
    private static get_arrow_control;
    private static get_scrollview_control;
    private static get_options_value;
    private static set_options_value;
    private static clear_options;
    private static remove_options;
    private static add_options;
    private static get_options;
    private static get_options_count;
}
declare class List<T> {
    private _items;
    private _has_equals_to;
    private _type;
    constructor(type: Traits_Constructor<T>);
    get(index: number): Traits_PrimitiveType<T>;
    get items(): Traits_PrimitiveType<T>[];
    get count(): number;
    private _IndexOfWithEqualsTo;
    IndexOf(val: Traits_PrimitiveType<T>): number;
    Add(item: Traits_PrimitiveType<T>): void;
    AddRange(objects: Traits_PrimitiveType<T>[] | List<T>): void;
    Insert(index: number, item: Traits_PrimitiveType<T>): void;
    InsertRange(index: number, objects: Traits_PrimitiveType<T>[] | List<T>): void;
    RemoveAt(index: number): void;
    Remove(val: Traits_PrimitiveType<T>): void;
    RemoveRange(index: number, count: number): void;
    Reverse(): void;
    Sort(sort_function?: (a: Traits_PrimitiveType<T>, b: Traits_PrimitiveType<T>) => number): void;
    Clear(): void;
    Swap(rhs: List<T>): void;
    SerializeJson(obj: any): void;
    DeserializeJson(obj: any): void;
    toString(): string;
    forEach(callback: (item: Traits_PrimitiveType<T>) => void): void;
    [Symbol.iterator](): Generator<Traits_PrimitiveType<T>, void, unknown>;
}
declare class Scrollbar extends TransitionControl {
    constructor(handle: never);
    get bg(): Control;
    get thumb(): Control;
    get value(): number;
    get size(): number;
    get direction(): SliderDirection;
    set value(value: number);
    set size(value: number);
    set direction(value: SliderDirection);
    AddEvent(event: ScrollbarEvent | ControlBaseEvent | ControlEvent, callBack: IControlEventCallBack | IControlEventCallBackI | IControlEventCallBackS): void;
    ClearEvent(event: ScrollbarEvent | ControlBaseEvent | ControlEvent, callBack?: IControlEventCallBack | IControlEventCallBackI | IControlEventCallBackS | null): void;
    private static handle_check;
    private static get_bg_control;
    private static get_thumb_control;
    private static get_scrollbar_value;
    private static get_scrollbar_size;
    private static get_scrollbar_direction;
    private static set_scrollbar_value;
    private static set_scrollbar_size;
    private static set_scrollbar_direction;
}
declare class NetDBGetter {
    private _handle;
    constructor(handle: never);
    get isDone(): boolean;
    GetValue(user_id: number, key: string): string;
    Request(user_id: number, key: string): void;
    Query(): Promise<void>;
    private static handle_check;
    private static get_is_query_done;
    private static get_value_impl;
    private static request_impl;
    private static query_impl;
}
declare class SceneManager {
    static get activeScene(): Scene;
    static get mainScene(): Scene;
    static set activeScene(scene: Scene);
    static set mainScene(scene: Scene);
    static CreateScene(name: String): Scene;
    static GetSceneByName(name: String): Scene;
    static UnloadScene(scene: Scene): boolean;
    static UnloadSceneByName(name: String): boolean;
    private static get_active_scene_impl;
    private static get_main_scene_impl;
    private static set_active_scene_impl;
    private static set_main_scene_impl;
    private static unload_scene_impl;
}
declare class RaycastHit {
    private _this_cld_id;
    private _this_transform_id;
    private _this_rigid_id;
    private _collider;
    private _transform;
    private _rigidbody;
    private _point;
    private _normal;
    private _texcoord;
    private _distance;
    get collider(): Collider;
    get transform(): Transform;
    get rigidbody(): Rigidbody;
    get point(): Vector3;
    get normal(): Vector3;
    get textureCoord(): Vector2;
    get distance(): number;
    private static CreateFromInternalBuffer;
    private static get_collider_by_id_impl;
    private static get_transform_by_id_impl;
    private static get_rigidbody_by_id_impl;
}
declare class Stack<T> {
    private _items;
    private _type;
    constructor(type: Traits_Constructor<T>);
    get(index: number): Traits_PrimitiveType<T>;
    get items(): Traits_PrimitiveType<T>[];
    get count(): number;
    Push(item: Traits_PrimitiveType<T>): void;
    Pop(): Traits_PrimitiveType<T>;
    Peek(): Traits_PrimitiveType<T>;
    Clear(): void;
    Swap(rhs: Stack<T>): void;
    SerializeJson(obj: any): void;
    DeserializeJson(obj: any): void;
    toString(): string;
    forEach(callback: (item: Traits_PrimitiveType<T>) => void): void;
    [Symbol.iterator](): Generator<Traits_PrimitiveType<T>, void, unknown>;
}
declare class Mesh extends AssetObject {
    constructor(handle?: never);
    get isReadable(): boolean;
    get isDynamic(): boolean;
    get bonesPerVertex(): BonesPerVertex;
    get indexFormat(): IndexFormat;
    get vertexCount(): number;
    get indexCount(): number;
    get subMeshCount(): number;
    get bounds(): Bounds;
    get triangles(): number[];
    get vertices(): Vector3[];
    get normals(): Vector3[];
    get tangents(): Vector4[];
    get colors(): Color[];
    get uv1(): Vector2[];
    get uv2(): Vector2[];
    get uv3(): Vector2[];
    get uv4(): Vector2[];
    get uv5(): Vector2[];
    get uv6(): Vector2[];
    get uv7(): Vector2[];
    get uv8(): Vector2[];
    get boneWeights(): BoneWeight[];
    get bindPoses(): Matrix4x4[];
    get bindPoseIndices(): number[];
    get subMeshes(): SubMeshDescriptor[];
    set bounds(bounds: Bounds);
    set triangles(indecies: number[]);
    set vertices(verts: Vector3[]);
    set normals(nors: Vector3[]);
    set tangents(tans: Vector4[]);
    set colors(cols: Color[]);
    set uv1(uv: Vector2[]);
    set uv2(uv: Vector2[]);
    set uv3(uv: Vector2[]);
    set uv4(uv: Vector2[]);
    set uv5(uv: Vector2[]);
    set uv6(uv: Vector2[]);
    set uv7(uv: Vector2[]);
    set uv8(uv: Vector2[]);
    set boneWeights(weights: BoneWeight[]);
    set bindPoses(poses: Matrix4x4[]);
    set bindPoseIndices(indices: number[]);
    set subMeshes(sub_meshes: SubMeshDescriptor[]);
    MarkDynamic(dynamic: boolean): void;
    Update(no_longer_readable: boolean): void;
    RecalculateBounds(): void;
    RecalculateNormals(): void;
    private static alloc;
    private static handle_check;
    private static get_is_readable_impl;
    private static get_is_dynamic_impl;
    private static get_bones_per_vertex;
    private static get_index_format;
    private static get_vertex_count;
    private static get_index_count;
    private static get_submesh_count;
    private static get_bounds_impl;
    private static get_triangles_impl;
    private static get_vertices_impl;
    private static get_normals_impl;
    private static get_tangents_impl;
    private static get_colors_impl;
    private static get_uv1_impl;
    private static get_uv2_impl;
    private static get_uv3_impl;
    private static get_uv4_impl;
    private static get_uv5_impl;
    private static get_uv6_impl;
    private static get_uv7_impl;
    private static get_uv8_impl;
    private static get_bone_weights_impl;
    private static get_bind_poses_impl;
    private static get_bind_pose_indecies_impl;
    private static get_sub_meshes;
    private static set_bounds_impl;
    private static set_triangles_impl;
    private static set_vertices_impl;
    private static set_normals_impl;
    private static set_tangents_impl;
    private static set_colors_impl;
    private static set_uv1_impl;
    private static set_uv2_impl;
    private static set_uv3_impl;
    private static set_uv4_impl;
    private static set_uv5_impl;
    private static set_uv6_impl;
    private static set_uv7_impl;
    private static set_uv8_impl;
    private static set_bone_weights_impl;
    private static set_bind_poses_impl;
    private static set_bind_pose_indecies_impl;
    private static set_sub_meshes;
    private static mark_dynamic_impl;
    private static update_impl;
    private static recl_bnds_impl;
    private static recl_normals_impl;
}
declare class OffMeshLink extends Component {
    constructor(handle: never);
    get startTransform(): Transform;
    get endTransform(): Transform;
    get biDirectional(): boolean;
    get area(): number;
    get activated(): boolean;
    set startTransform(target: Transform);
    set endTransform(target: Transform);
    set biDirectional(val: boolean);
    set area(val: number);
    set activated(val: boolean);
    private static handle_check;
    private static get_start_impl;
    private static get_end_impl;
    private static get_is_bidirection_impl;
    private static get_area_impl;
    private static get_activated_impl;
    private static set_start_impl;
    private static set_end_impl;
    private static set_is_bidirection_impl;
    private static set_area_impl;
    private static set_activated_impl;
}
declare class SDKs {
    static Invoke(name: string, ...args: string[]): Promise<SDKResult>;
    static InvokeSync(name: string, callback: (SDKResult: any) => void, ...args: string[]): void;
    private static invoke_impl;
    private static invoke_sync_impl;
}
declare class Time {
    static get realTimeSinceStartUp(): number;
    static get frameCount(): number;
    static get frameTime(): number;
    static get timeScale(): number;
    static get deltaTime(): number;
    static get unscaledDeltaTime(): number;
    static get fixedDeltaTime(): number;
    static get invDeltaTime(): number;
    static set timeScale(scale: number);
    private static rt_st_impl;
    private static frame_count_impl;
    private static frame_time_impl;
    private static time_scale_impl;
    private static dt_impl;
    private static unscl_dt_impl;
    private static fixed_dt_impl;
    private static inv_dt_impl;
    private static set_time_scale_impl;
}
declare class PhysicMaterial extends AssetObject {
    constructor(handle?: never);
    get bounciness(): number;
    get dynamicFriction(): number;
    get staticFriction(): number;
    get frictionCombine(): PhysicMaterialCombine;
    get bounceCombine(): PhysicMaterialCombine;
    set bounciness(bounciness: number);
    set dynamicFriction(dynamicFri: number);
    set staticFriction(staticFri: number);
    set frictionCombine(dynamicFri: PhysicMaterialCombine);
    set bounceCombine(dynamicFri: PhysicMaterialCombine);
    private static alloc;
    private static handle_check;
    private static getBounciness;
    private static getDynamicFriction;
    private static getStaticFriction;
    private static getFrictionCombine;
    private static getBounceCombine;
    private static setBounciness;
    private static setDynamicFriction;
    private static setStaticFriction;
    private static setFrictionCombine;
    private static setBounceCombine;
}
declare class CSV extends AssetObject {
    private constructor();
    get rowsCount(): number;
    lineCount(row: number): number;
    SetData(val: string): void;
    GetItem(row: number, col: number): string;
    private static handle_check;
    private static get_rows_count_impl;
    private static get_line_count_impl;
    private static set_data_impl;
    private static get_item_impl;
}
declare class Transform extends EngineObject {
    private _handle;
    constructor(handle: never);
    get instanceID(): number;
    get name(): string;
    get gameObject(): GameObject;
    get up(): Vector3;
    get right(): Vector3;
    get forward(): Vector3;
    get childCount(): number;
    get parent(): Transform;
    get localPosition(): Vector3;
    get localScale(): Vector3;
    get localRotation(): Quaternion;
    get position(): Vector3;
    get lossyScale(): Vector3;
    get rotation(): Quaternion;
    get eulerAngles(): Vector3;
    get localEulerAngles(): Vector3;
    get localToWorldMatrix(): Matrix4x4;
    get worldToLocalMatrix(): Matrix4x4;
    set parent(transform: Transform);
    set localPosition(pos: Vector3);
    set localScale(scale: Vector3);
    set localRotation(rot: Quaternion);
    set position(pos: Vector3);
    set rotation(rot: Quaternion);
    SetParent(parent: Transform, keep_world_position?: boolean): void;
    FindChild(name: string): Transform;
    GetChild(index: number): Transform;
    LookAt(pos: Vector3, up?: Vector3): void;
    SetLocalTR(pos: Vector3, rot: Quaternion): void;
    SetLocalTRS(pos: Vector3, rot: Quaternion, scale: Vector3): void;
    GetPositionAndRotation(ref_pos: Vector3, ref_rot: Quaternion): void;
    SetPositionAndRotation(pos: Vector3, rot: Quaternion): void;
    TransformVector(vector: Vector3): Vector3;
    TransformPoint(point: Vector3): Vector3;
    InverseTransformVector(vector: Vector3): Vector3;
    InverseTransformPoint(point: Vector3): Vector3;
    RotateEuler(euler: Vector3, relative_to_local?: boolean): void;
    RotateEulerXYZ(euler_x: number, euler_y: number, euler_z: number, relative_to_local?: boolean): void;
    toString(): string;
    forEach(callback: (item: Transform) => void): void;
    [Symbol.iterator](): Generator<Transform, void, unknown>;
    private static handle_check;
    private static get_or_alloc_gameobject;
    private static get_instance_id_impl;
    private static get_name_impl;
    private static get_up_impl;
    private static get_right_impl;
    private static get_forward_impl;
    private static get_childCount_impl;
    private static get_parent_impl;
    private static get_lcl_pos_impl;
    private static get_lcl_scl_impl;
    private static get_lcl_rot_impl;
    private static get_pos_impl;
    private static get_scl_impl;
    private static get_rot_impl;
    private static get_euler_impl;
    private static get_l2w_impl;
    private static get_w2l_impl;
    private static set_parent_impl;
    private static set_lcl_pos_impl;
    private static set_lcl_scl_impl;
    private static set_lcl_rot_impl;
    private static set_pos_impl;
    private static set_rot_impl;
    private static find_child_impl;
    private static get_child_impl;
    private static look_at_impl;
    private static set_local_tr_impl;
    private static set_local_trs_impl;
    private static get_tr_impl;
    private static set_tr_impl;
    private static trans_vec_impl;
    private static trans_point_impl;
    private static inv_trans_vec_impl;
    private static inv_trans_point_impl;
}
declare class HorizontalLayout extends LayoutGroup {
    constructor(handle: never);
    get spacing(): number;
    get reverse(): boolean;
    set spacing(spacing: number);
    set reverse(enable: boolean);
    private static handle_check;
    private static get_spacing;
    private static get_reverse;
    private static set_spacing;
    private static set_reverse;
}
declare class Collision {
    private _map_flags;
    private _other_cld_id;
    private _other_rig_id;
    private _other_go;
    private _other_collider;
    private _other_rigidbody;
    private _impulse;
    private _relative_velocity;
    private _contacts;
    private get _otherColliderGetter();
    get collider(): Collider;
    get rigidbody(): Rigidbody;
    get gameObject(): GameObject;
    get transform(): Transform;
    get contactPoints(): ContactPoint[];
    get contactCount(): number;
    get impulse(): Vector3;
    get relativeVelocity(): Vector3;
    GetContactPoint(index: number): ContactPoint;
    private static CreateFromInternalBuffer;
    private static get_collider_by_id_impl;
    private static get_rigidbody_by_id_impl;
}
interface IContractEchocapable {
    Respond(...args: any): void;
}
declare class Prefab extends AssetObject {
    private constructor();
    get uuid(): string;
    Instance(): GameObject;
    private static handle_check;
    private static inst_impl;
    private static get_uuid_impl;
}
declare abstract class Contract {
    private _contracter;
    private _origin;
    private _state;
    get origin(): NetOrigin;
    get conversation(): NetConversation;
    protected Send<T extends Contract>(type: new (...args: any[]) => T, ...args: Parameters<T['Receive']>): void;
    protected Echo<T extends (Contract & IContractEchocapable)>(type: new (...args: any[]) => T, ...args: Parameters<T['Respond']>): void;
    protected FindEntity(entityID: string): NetEntity;
    protected CreateNewEntity(prefab_id: string, userdata: string, callback?: (entity: NetEntity) => void): void;
    abstract Exescute(...args: any): void;
    abstract Receive(...args: any): void;
    private Init;
    private static OnContractReceive;
    private static set_contract_impl;
}
declare class Slider extends TransitionControl {
    constructor(handle: never);
    get bg(): Control;
    get fill(): Control;
    get thumb(): Control;
    get wholeNumbers(): boolean;
    get minValue(): number;
    get maxValue(): number;
    get value(): number;
    get direction(): SliderDirection;
    get sliderType(): SliderType;
    set wholeNumbers(enable: boolean);
    set minValue(value: number);
    set maxValue(value: number);
    set value(value: number);
    set direction(direction: SliderDirection);
    set sliderType(type: SliderType);
    AddEvent(event: SliderEvent | ControlBaseEvent | ControlEvent, callBack: IControlEventCallBack | IControlEventCallBackI | IControlEventCallBackS): void;
    ClearEvent(event: SliderEvent | ControlBaseEvent | ControlEvent, callBack?: IControlEventCallBack | IControlEventCallBackI | IControlEventCallBackS | null): void;
    private static handle_check;
    private static get_bg_control;
    private static get_fill_control;
    private static get_thumb_control;
    private static get_whole_numbers;
    private static get_min_value;
    private static get_max_value;
    private static get_value;
    private static get_direction;
    private static get_slider_type;
    private static set_whole_numbers;
    private static set_min_value;
    private static set_max_value;
    private static set_value;
    private static set_direction;
    private static set_slider_type;
}
declare class ContactPoint {
    private _point;
    private _normal;
    private _separation;
    private _collision;
    constructor(owner: Collision);
    get point(): Vector3;
    get normal(): Vector3;
    get separation(): number;
    get thisCollider(): Collider;
    get otherCollider(): Collider;
}
declare class BehaviorBool extends BehaviorVariable {
    private constructor();
    GetValue(): boolean;
    SetValue(val: boolean): void;
    static Create(): BehaviorBool;
    private static set_val;
    private static get_val;
}
declare class StartBehaviorTree extends BehaviorAction {
    constructor(handle?: never);
    get behaviorGameObject(): BehaviorGameObject;
    get waitForCompletion(): BehaviorBool;
    get synchronizeVariables(): BehaviorBool;
    set behaviorGameObject(val: BehaviorGameObject | GameObject);
    set waitForCompletion(val: BehaviorBool);
    set synchronizeVariables(val: BehaviorBool);
    SetBehaviorGameObject(obj: GameObject): void;
    SetWaitForCompletion(val: boolean): void;
    SetSynchronizeVariables(val: boolean): void;
    private static get_behavior;
    private static get_wait_for_completion;
    private static get_sync_var;
    private static set_behavior;
    private static set_wait_for_completion;
    private static set_sync;
    private static set_behavior_var;
    private static set_wait_for_completion_var;
    private static set_sync_var;
}
declare class RenderTexture extends Texture {
    constructor(handle?: never);
    get antiAliasing(): AntiAliasingLevel;
    get renderTextureFormat(): RenderTextureFormat;
    get depthType(): DepthType;
    get depthStencilFormat(): GraphicsFormat;
    GetPixel(x: number, y: number, mipLevel: number): Color;
    GetPixels(x: number, y: number, blockWidth: number, blockHeight: number, mipLevel?: number): Color[] | undefined;
    SetPixel(x: number, y: number, col: Color, mipLevel: number): void;
    SetPixels(x: number, y: number, blockWidth: number, blockHeight: number, colors: Color[], mipLevel?: number): void;
    Apply(mipLevel?: number): void;
    ReleaseCPUMemory(): void;
    SavePngToFile(filePath: string): void;
    static get active(): RenderTexture;
    static set active(val: RenderTexture);
    static GetTemporary(width: number, height: number, colorFmt: RenderTextureFormat, depth: DepthType, aa?: AntiAliasingLevel): RenderTexture | null;
    static ReleaseTemporary(rt: RenderTexture): void;
    static CreateRenderTexture(width: number, height: number, colorFmt: RenderTextureFormat, depth: DepthType, aa?: AntiAliasingLevel, mip?: boolean): RenderTexture;
    private static handle_check;
    private static get_anti_aliasing_impl;
    private static get_render_texture_format_impl;
    private static get_depth_type_impl;
    private static get_depth_stencil_format_impl;
    private static get_active_impl;
    private static set_active_impl;
    private static get_temporary_impl;
    private static release_temporary_impl;
    private static get_pixel_impl;
    private static get_pixels_impl;
    private static set_pixel_impl;
    private static set_pixels_impl;
    private static apply_impl;
    private static release_cpu_memory_impl;
    private static create_render_texture_impl;
    private static save_png_to_file_impl;
}
declare class IntPtr {
    constructor();
    toString(): string;
    static Check(ptr: IntPtr): boolean;
}
declare class Scene extends EngineObject {
    private _handle;
    constructor(handle: never);
    get instanceID(): number;
    get name(): string;
    get gameObjectCount(): number;
    set name(name: string);
    IsGameObjectExists(go: GameObject): boolean;
    Reload(): boolean;
    private static handle_check;
    private static get_instance_id_impl;
    private static get_name_impl;
    private static get_go_count_impl;
    private static set_name_impl;
    private static go_exists_impl;
    private static reload_impl;
}
declare class ControllerColliderHit {
    private _map_flags;
    private _cld_id;
    private _ctrl_id;
    private _collider;
    private _controller;
    private _go;
    private _point;
    private _normal;
    private _move_dir;
    private _move_len;
    get collider(): Collider;
    get controller(): CharacterController;
    get gameObject(): GameObject;
    get transform(): Transform;
    get point(): Vector3;
    get normal(): Vector3;
    get moveDirection(): Vector3;
    get moveLength(): number;
    private static CreateFromInternalBuffer;
    private static get_collider_by_id_impl;
    private static get_controller_by_id_impl;
}
declare class Physics {
    static get isQueriesHitTriggers(): boolean;
    static get isQueriesHitBackfaces(): boolean;
    static collider_by_id(id: number): Collider;
    static Raycast(ray: Ray, maxDistance: number, mask: number, cast_trigger?: QueryTriggerInteraction): RaycastHit | undefined;
    static RaycastAll(ray: Ray, maxDistance: number, mask: number, cast_trigger?: QueryTriggerInteraction): RaycastHit[] | undefined;
    static SphereCast(ray: Ray, radius: number, maxDistance: number, mask: number, cast_trigger?: QueryTriggerInteraction): RaycastHit | undefined;
    static SphereCastAll(ray: Ray, radius: number, maxDistance: number, mask: number, cast_trigger?: QueryTriggerInteraction): RaycastHit[] | undefined;
    static CapsuleCast(p0: Vector3, p1: Vector3, radius: number, direction: Vector3, maxDistance: number, mask: number, cast_trigger?: QueryTriggerInteraction): RaycastHit | undefined;
    static CapsuleCastAll(p0: Vector3, p1: Vector3, radius: number, direction: Vector3, maxDistance: number, mask: number, cast_trigger?: QueryTriggerInteraction): RaycastHit[] | undefined;
    static BoxCast(center: Vector3, extents: Vector3, rotation: Quaternion, direction: Vector3, maxDistance: number, mask: number, cast_trigger?: QueryTriggerInteraction): RaycastHit | undefined;
    static BoxCastAll(center: Vector3, extents: Vector3, rotation: Quaternion, direction: Vector3, maxDistance: number, mask: number, cast_trigger?: QueryTriggerInteraction): RaycastHit[] | undefined;
    static OverlapBox(center: Vector3, halfExtents: Vector3, orientation: Quaternion, mask: number, query_trigger?: QueryTriggerInteraction): Collider[] | undefined;
    static OverlapCapsule(point0: Vector3, point1: Vector3, radius: number, mask: number, query_trigger?: QueryTriggerInteraction): Collider[] | undefined;
    static OverlapSphere(position: Vector3, radius: number, mask: number, query_trigger?: QueryTriggerInteraction): Collider[] | undefined;
    private static getIsQueriesHitTriggers_impl;
    private static getIsQueriesHitBackfaces_impl;
    private static raycast_impl;
    private static raycastAll_impl;
    private static sphereCast_impl;
    private static sphereCastAll_impl;
    private static capsuleCast_impl;
    private static capsuleCastAll_impl;
    private static boxCast_impl;
    private static boxCastAll_impl;
    private static overlapBox_impl;
    private static overlapCapsule_impl;
    private static overlapSphere_impl;
    private static get_collider_by_id_impl;
}
declare class GUIMask extends Control {
    constructor(handle: never);
    get texture(): Texture | null;
    get edgeNum(): number;
    get maskType(): MaskType;
    get reverse(): boolean;
    get colorFill(): boolean;
    get frameOutline(): boolean;
    get softnessX(): number;
    get softnessY(): number;
    get frameWidth(): number;
    get fillColor(): Color;
    get frameColor(): Color;
    get fillPadding(): Vector4;
    get vertexDistance(): number[];
    get uvRegion(): Rect;
    get imageUV(): Rect;
    get maskUV(): Rect;
    get maskRect(): Rect;
    get vertexList(): Vector3[];
    get borderVertexList(): Vector3[];
    set texture(texture: Texture);
    set maskType(value: MaskType);
    set reverse(value: boolean);
    set colorFill(value: boolean);
    set frameOutline(value: boolean);
    set softnessX(value: number);
    set softnessY(value: number);
    set frameWidth(value: number);
    set fillColor(value: Color);
    set frameColor(value: Color);
    set fillPadding(value: Vector4);
    set vertexDistance(value: number[]);
    SetAtlasTexture(atlasUUID: string, textureUUID: string): void;
    PointInMask(value: Vector2): boolean;
    AddEvent(event: GUIMaskEvent | ControlBaseEvent | ControlEvent, callBack: IControlEventCallBack | IControlEventCallBackI | IControlEventCallBackS): void;
    ClearEvent(event: GUIMaskEvent | ControlBaseEvent | ControlEvent, callBack?: IControlEventCallBack | IControlEventCallBackI | IControlEventCallBackS | null): void;
    private static handle_check;
    private static get_texture;
    private static get_edge_num;
    private static get_mask_type;
    private static get_reverse;
    private static get_color_fill;
    private static get_frame_outline;
    private static get_softness_x;
    private static get_softness_y;
    private static get_frame_width;
    private static get_fill_color;
    private static get_frame_color;
    private static get_fill_padding;
    private static get_vertex_distance;
    private static get_uv_region;
    private static get_image_uv;
    private static get_mask_uv;
    private static get_mask_rect;
    private static get_vertex_list;
    private static get_border_vertex_list;
    private static set_texture;
    private static set_mask_type;
    private static set_reverse;
    private static set_color_fill;
    private static set_frame_outline;
    private static set_softness_x;
    private static set_softness_y;
    private static set_frame_width;
    private static set_fill_color;
    private static set_frame_color;
    private static set_fill_padding;
    private static set_vertex_distance;
    private static set_atlas_texture;
    private static point_in_mask;
}
declare class Rect {
    x: number;
    y: number;
    width: number;
    height: number;
    constructor(x?: number, y?: number, w?: number, h?: number);
    get area(): number;
    get min(): Vector2;
    get max(): Vector2;
    SetValues(x: number, y: number, w: number, h: number): void;
    SetMinMax(min: Vector2, max: Vector2): void;
    Overlaps(other: Rect): boolean;
    Contains(point: Vector2): boolean;
    Scale(scale_x: number, scale_y: number): void;
    Move(delta_x: number, delta_y: number): void;
    EncapsulatePoint(point: Vector2): void;
    EncapsulateRect(rect: Rect): void;
    toString(): string;
    CopyFrom(from: Rect): void;
    EqualsTo(other: Rect): boolean;
    static Clone(val: Rect): Rect;
    static EqualsTo(lhs: Rect, rhs: Rect): boolean;
}
declare class ExternalBehaviorTree extends AssetObject {
    private constructor();
    get BehaviorSource(): BehaviorSource | null;
    private static handle_check;
    private static get_behavior_src;
}
declare class Resources {
    static Load<T extends AssetObject>(type: Traits_Constructor<T>, id: string): T;
    static LoadAsync(id: string, callback?: IOnResourceLoaded): boolean;
    static LoadAwait<T extends AssetObject>(type: Traits_Constructor<T>, id: string): Promise<T>;
    private static load_sync_impl;
    private static load_async_impl;
}
declare class TextBox extends TransitionControl {
    constructor(handle: never);
    get bg(): Control;
    get label(): Control;
    get placeholder(): Control;
    get text(): string;
    get readOnly(): boolean;
    get numLimit(): number;
    get cursorColor(): Color;
    get selectColor(): Color;
    get contentLimit(): ContentLimit;
    get returnType(): InputReturnType;
    set text(text: string);
    set readOnly(enable: boolean);
    set numLimit(limit: number);
    set cursorColor(color: Color);
    set selectColor(color: Color);
    set contentLimit(limit: ContentLimit);
    set returnType(value: InputReturnType);
    AddEvent(event: TextBoxEvent | ControlBaseEvent | ControlEvent, callBack: IControlEventCallBack | IControlEventCallBackI | IControlEventCallBackS): void;
    ClearEvent(event: TextBoxEvent | ControlBaseEvent | ControlEvent, callBack?: IControlEventCallBack | IControlEventCallBackI | IControlEventCallBackS | null): void;
    private static handle_check;
    private static get_bg_control;
    private static get_label_control;
    private static get_placeholder_control;
    private static get_text;
    private static get_read_only;
    private static get_num_limit;
    private static get_cursor_color;
    private static get_select_color;
    private static get_content_limit;
    private static get_return_type;
    private static set_text;
    private static set_read_only;
    private static set_num_limit;
    private static set_cursor_color;
    private static set_select_color;
    private static set_content_limit;
    private static set_return_type;
}
declare class CurveUtils {
    static SmoothTangents(curve: Curve, index?: number, weight?: number): void;
    private static smooth_tangents;
}
declare class Debug {
    static set ExceptionCallback(callback: (string: any) => void);
    static Log(...args: any[]): void;
    static Warning(...args: any[]): void;
    static Error(...args: any[]): void;
    static GetSourceMapURL(path: string): string;
    static SetTraceLog(value: boolean): void;
    private static __WaitForDebugerForEngine;
    private static log_impl;
    private static warning_impl;
    private static error_impl;
    private static get_sourcemap_url_impl;
    private static set_trace_log;
    private static set_exception_callback_impl;
}
declare class DictionaryItem<K, V> {
    private _key;
    value: Traits_PrimitiveType<V>;
    constructor(key: Traits_PrimitiveType<K>, value: Traits_PrimitiveType<V>);
    get key(): Traits_PrimitiveType<K>;
}
declare class StopBehaviorTree extends BehaviorAction {
    constructor(handle?: never);
    get behaviorGameObject(): BehaviorGameObject;
    get pauseBehavior(): BehaviorBool;
    set behaviorGameObject(val: BehaviorGameObject | GameObject);
    set waitForCompletion(val: BehaviorBool);
    set pauseBehavior(val: BehaviorBool);
    SetBehaviorGameObject(obj: GameObject): void;
    SetPauseBehavior(val: boolean): void;
    private static get_behavior;
    private static get_pause_behavior;
    private static set_behavior;
    private static set_pause_behavior;
    private static set_behavior_var;
    private static set_pause_behavior_var;
}
declare class ScenePrefab extends AssetObject {
    private constructor();
    Instance(): Scene;
    private static handle_check;
    private static inst_impl;
}
declare class Delegate<T extends (...args: any[]) => any> {
    private _funcs;
    private _locked;
    private _remove_queue;
    get count(): number;
    get invokable(): boolean;
    private _IndexOf;
    private _ClearLockQueue;
    Add<Class>(func: T, obj?: Class): void;
    Remove<Class>(func: T, obj?: Class): void;
    Clear(): void;
    Invoke(...args: Parameters<T>): ReturnType<T>;
    toString(): string;
    Clone(): Delegate<T>;
}
declare class Sprite extends AssetObject {
    private constructor();
    get spriteRect(): Rect;
    get spriteUV(): Rect;
    get pivot(): Vector2;
    set pivot(value: Vector2);
    private static handle_check;
    private static get_sprite_rect;
    private static get_sprite_uv;
    private static get_pivot;
    private static set_pivot;
}
declare class LineRenderer extends Renderer {
    constructor(handle: never);
    get alignment(): LineAlignment;
    get colorGradient(): Gradient;
    get endColor(): Color;
    get endWidth(): number;
    get generateLightingData(): boolean;
    get loop(): boolean;
    get numCapVertices(): number;
    get numCornerVertices(): number;
    get positionCount(): number;
    get startColor(): Color;
    get startWidth(): number;
    get textrueMode(): LineTextureMode;
    get useWorldSpace(): boolean;
    get widthCurve(): Curve;
    get widthMultiplier(): number;
    set alignment(align: LineAlignment);
    set colorGradient(color: Gradient);
    set endColor(color: Color);
    set endWidth(width: number);
    set generateLightingData(gen: boolean);
    set loop(is_loop: boolean);
    set numCapVertices(num: number);
    set numCornerVertices(num: number);
    set positionCount(count: number);
    set startColor(color: Color);
    set startWidth(width: number);
    set textrueMode(mode: LineTextureMode);
    set useWorldSpace(use: boolean);
    set widthCurve(width_curve: Curve);
    set widthMultiplier(mul: number);
    GetPosition(index: number): Vector3;
    GetPositions(): Vector3[];
    SetPosition(index: number, pos: Vector3): void;
    SetPositions(pos: Vector3[]): void;
    Simplify(): void;
    private static handle_check;
    private static get_alignment_impl;
    private static get_color_gradient_impl;
    private static get_gen_light_data_impl;
    private static get_loop_impl;
    private static get_num_cap_vertices_impl;
    private static get_num_corner_vertices_impl;
    private static get_position_count_impl;
    private static get_texture_mode_impl;
    private static get_use_world_space_impl;
    private static get_width_curve_impl;
    private static get_width_multiplier_impl;
    private static set_alignment_impl;
    private static set_color_gradient_impl;
    private static set_gen_light_data_impl;
    private static set_loop_impl;
    private static set_num_cap_vertices_impl;
    private static set_num_corner_vertices_impl;
    private static set_position_count_impl;
    private static set_texture_mode_impl;
    private static set_use_world_space_impl;
    private static set_width_curve_impl;
    private static set_width_multiplier_impl;
    private static get_position_impl;
    private static get_positions_impl;
    private static set_position_impl;
    private static set_positions_impl;
    private static simplify_impl;
}
declare class Exception extends Error {
    name: string;
}
declare class Shader extends AssetObject {
    private constructor();
    static Find(shader_name: string): Shader | undefined;
    private static handle_check;
    private static find_impl;
}
declare class Input {
    static get mouseDeltaPosition(): Vector2;
    static get mousePosition(): Vector2;
    static get mouseScrollDelta(): Vector2;
    static get inputString(): string;
    static get touchCount(): number;
    static GetKey(key: KeyCode): boolean;
    static GetKeyDown(key: KeyCode): boolean;
    static GetKeyUp(key: KeyCode): boolean;
    static GetMouseButton(button: MouseButton): boolean;
    static GetMouseButtonDown(button: MouseButton): boolean;
    static GetMouseButtonUp(button: MouseButton): boolean;
    static GetTouch(index: number): Touch;
    static GetAxis(pad_index: number, axis: GamePadAxis): number;
    private static get_m_delta_pos;
    private static get_m_pos;
    private static get_m_scroll_delta;
    private static get_key_impl;
    private static get_key_down_impl;
    private static get_key_up_impl;
    private static get_mouse_impl;
    private static get_mouse_down_impl;
    private static get_mouse_up_impl;
    private static get_input_string_impl;
    private static get_touch_count_impl;
}
declare class ObjectDictionary<K extends (IComparable & IHashCodeProvider), V> {
    private _count;
    private _null_added;
    private _null_value;
    private _handle;
    get count(): number;
    Get(key: Traits_PrimitiveType<K>): Traits_PrimitiveType<V>;
    Set(key: Traits_PrimitiveType<K>, value: Traits_PrimitiveType<V>): void;
    Add(key: Traits_PrimitiveType<K>, value: Traits_PrimitiveType<V>): void;
    Remove(key: Traits_PrimitiveType<K>): void;
    Contains(key: Traits_PrimitiveType<K>): boolean;
    Clear(): void;
    toString(): string;
    forEach(callback: (key: K, value: V) => void): void;
    [Symbol.iterator](): Generator<DictionaryItem<K, V>, void, unknown>;
}
declare class Skeleton extends AssetObject {
    private constructor();
    private static handle_check;
}
declare class SubMeshDescriptor {
    bounds: Bounds;
    indexStart: number;
    indexCount: number;
    baseVertex: number;
    firstVertex: number;
    vertexCount: number;
}
declare class NetInstanceToken {
    private _state;
    private _instance_scene_id;
    get state(): NetInstanceState;
    get scene_id(): string;
    set state(value: NetInstanceState);
    set scene_id(value: string);
    private SendToInternalBuffer;
    private static CreateFromInternalBuffer;
}
declare class Platform {
    static get isMobilePlatform(): boolean;
    static get engineOrigin(): EngineOrigin;
    private static get_is_mobile_impl;
    private static get_engine_origin_impl;
}
declare class SceneLoadingPanelProcessor {
    static EnterSceneByUUID(uuid: string): boolean;
    private static enter_Scene_ByUUID;
}
declare enum ScreenOrientation {
    Portrait = 1,
    PortraitUpsideDown = 2,
    LandscapeLeft = 4,
    LandscapeRight = 8,
    PortraitSensor = 3,
    LandscapeSensor = 12,
    AutoRotation = 15
}
declare class Screen {
    private static _orientation;
    static get DPI(): number;
    static get width(): number;
    static get height(): number;
    static get pixelScale(): number;
    static get orientation(): ScreenOrientation;
    static set orientation(value: ScreenOrientation);
    private static get_dpi_impl;
    private static get_width_impl;
    private static get_height_impl;
    private static get_pixel_scale_impl;
    private static set_screen_orientation_impl;
}
declare class KartController extends Component {
    constructor(handle: never);
    get throttle(): number;
    get brake(): number;
    get steer(): number;
    get handbrake(): number;
    get boost(): number;
    get currentspeed(): number;
    get maxspeed(): number;
    get isground(): number;
    set throttle(value: number);
    set brake(value: number);
    set steer(value: number);
    set handbrake(value: number);
    set boost(value: number);
    set maxspeed(value: number);
    private static handle_check;
    private static getthrottle_impl;
    private static getbrake_impl;
    private static getsteer_impl;
    private static gethandbrake_impl;
    private static getboost_impl;
    private static getcurrentspeed_impl;
    private static getmaxspeed_impl;
    private static getisgrounded_impl;
    private static setthrottle_impl;
    private static setbrake_impl;
    private static setsteer_impl;
    private static sethandbrake_impl;
    private static setboost_impl;
    private static setmaxspeed_impl;
}
declare class ComponentSettings {
    static ExecuteOrder(order: number): (ctor: Function) => void;
    static ExecuteMultiple(): (ctor: Function) => void;
}
declare class SDKResult {
    message: any;
    success: boolean;
}
declare class RenderViewport extends EngineObject {
    private _handle;
    constructor(handle: never);
    get DPI(): number;
    get width(): number;
    get height(): number;
    get rendertarget(): RenderTexture | null;
    get isRenderingCurrent(): boolean;
    get pixelScale(): number;
    get antiAliasingLevel(): AntiAliasingLevel;
    get renderStatistics(): string;
    set pixelScale(value: number);
    set antiAliasingLevel(value: AntiAliasingLevel);
    private static handle_check;
    private static get_dpi_impl;
    private static get_width_impl;
    private static get_height_impl;
    private static get_rendertarget_impl;
    private static get_is_rendering_current_impl;
    private static get_pixel_scaling_impl;
    private static set_pixel_scaling_impl;
    private static get_anti_aliasing_level_impl;
    private static get_get_render_statistics_impl;
    private static set_anti_aliasing_level_impl;
}
declare class StopWatch {
    private _handle;
    constructor(handle?: never);
    Start(): void;
    Stop(): void;
    Restart(): void;
    get elapsedUs(): number;
    get elapsedMs(): number;
    get elapsed(): number;
    private static alloc_object;
    private static handle_check;
    private static Start_impl;
    private static Stop_impl;
    private static Restart_impl;
    private static Elapsed_impl;
    private static Elapsed_MS_impl;
    private static Elapsed_Second_impl;
}
declare class SpeechRecognize {
    static OnHealModelCallback(callback: IOnSpeechRecognizeCallBack): void;
    private static heal_model_callback;
}
declare class LookAtIK extends IKSolver {
    constructor(handle: never);
    get target(): Transform | null;
    get head(): Transform | null;
    get bodyWeight(): number;
    get headWeight(): number;
    get eyesWeight(): number;
    get clampWeightBody(): number;
    get clampWeightHead(): number;
    get clampWeightEyes(): number;
    set target(transform: Transform);
    set head(transform: Transform);
    set bodyWeight(weight: number);
    set headWeight(weight: number);
    set eyesWeight(weight: number);
    set clampWeightBody(weight: number);
    set clampWeightHead(weight: number);
    set clampWeightEyes(weight: number);
    private static handle_check;
    private static getTarget;
    private static getHead;
    private static getBodyWeight;
    private static getHeadWeight;
    private static getEyesWeight;
    private static getClampWeightBody;
    private static getClampWeightHead;
    private static getClampWeightEyes;
    private static setTarget;
    private static setHead;
    private static setBodyWeight;
    private static setHeadWeight;
    private static setEyesWeight;
    private static setClampWeightBody;
    private static setClampWeightHead;
    private static setClampWeightEyes;
}
declare class Graphics {
    static Blit(tex: Texture, rt: RenderTexture): void;
    static BlitWithMaterial(tex: Texture, rt: RenderTexture, material: Material): void;
    static FlipAndBlit(tex: Texture, rt: RenderTexture, fliped: boolean): void;
    static DrawMesh(world: Matrix4x4, mesh: Mesh, mtl: Material, shadow_cast?: ShadowCastingMode): void;
    static DrawMeshNow(world: Matrix4x4, mesh: Mesh, mtl: Material): void;
    static DrawRendererNow(renderer: Renderer, camera: Camera, replaced_material?: Material): void;
    static Clear(clearDepth: boolean, clearColor: boolean, backgroundColor: Color, depth?: number): void;
    private static blit_flip_impl;
    private static blit_mtl_impl;
    private static draw_mesh_impl;
    private static draw_mesh_now_impl;
    private static draw_renderer_now_impl;
    private static clear_impl;
}
interface StringConstructor {
    IsNullOrEmpty(str: string): boolean;
}
declare class ObjectHashSetBucket {
    hash: number;
    items: IComparable[];
    constructor(hash: number);
    Add(item: IComparable): boolean;
    Remove(item: IComparable): boolean;
    Contains(item: IComparable): boolean;
}
declare class Touch extends EngineObject {
    private _handle;
    constructor(handle: never);
    get phase(): TouchPhase;
    get tapCount(): number;
    get fingerID(): number;
    get deltaPosition(): Vector2;
    get position(): Vector2;
    get rawPosition(): Vector2;
    get touchMask(): number;
    private static handle_check;
    private static get_phase_impl;
    private static get_tap_impl;
    private static get_finger_id_impl;
    private static get_delta_pos_impl;
    private static get_pos_impl;
    private static get_raw_pos_impl;
    private static get_touch_mask;
}
declare class SpriteSequence extends AssetObject {
    private constructor();
    get frameRate(): number;
    set frameRate(value: number);
    private static handle_check;
    private static get_frame_rate;
    private static set_frame_rate;
}
declare class GUIEngine {
    static NewControl<T extends Control | LayoutGroup>(type: Traits_Constructor<T>, parent: Control): T | null;
    static IsExpand(control: Control): boolean;
    static get currentForm(): Form;
    private static get_current_form;
    private static new_control;
    private static is_expand;
}
declare class GUIEventData extends EngineObject {
    private _handle;
    constructor(handle: never);
    get eventType(): EventType;
    get button(): MouseButton;
    get buttonState(): ButtonState;
    get touchID(): number;
    get clickCount(): number;
    get position(): Vector2;
    get moveDelta(): Vector2;
    get wheelScrollDelta(): Vector2;
    get isNone(): boolean;
    toString(): string;
    private static handle_check;
    private static get_event_type;
    private static get_button;
    private static get_button_state;
    private static get_touch_id;
    private static get_click_count;
    private static get_position;
    private static get_move_delta;
    private static get_wheelscroll_delta;
    private static is_none;
}
declare class ValueDictionary<K extends Number | String | Boolean, V> {
    private _handle;
    get count(): number;
    Get(key: Traits_PrimitiveType<K>): Traits_PrimitiveType<V>;
    Set(key: Traits_PrimitiveType<K>, value: Traits_PrimitiveType<V>): void;
    Add(key: Traits_PrimitiveType<K>, value: Traits_PrimitiveType<V>): void;
    Remove(key: Traits_PrimitiveType<K>): void;
    Contains(key: Traits_PrimitiveType<K>): boolean;
    Clear(): void;
    toString(): string;
    forEach(callback: (key: Traits_PrimitiveType<K>, value: Traits_PrimitiveType<V>) => void): void;
    [Symbol.iterator](): Generator<DictionaryItem<K, V>, void, unknown>;
}
interface IComparable {
    EqualsTo(other: object): boolean;
}
declare class TextAsset extends AssetObject {
    private constructor();
    get data(): string;
    private static handle_check;
    private static get_str;
}
declare class Gizmos {
    static get color(): Color;
    static get matrix(): Matrix4x4;
    static get lineWidth(): number;
    static set color(col: Color);
    static set matrix(mat: Matrix4x4);
    static set lineWidth(val: number);
    static CleanMatrix(): void;
    static DrawCube(center: Vector3, size: Vector3): void;
    static DrawWireCube(center: Vector3, size: Vector3, depthTest?: boolean): void;
    static DrawLine(p1: Vector3, p2: Vector3, depthTest?: boolean): void;
    static DrawLineWithColor(p1: Vector3, p1_color: Color, p2: Vector3, p2_color: Color, depthTest?: boolean): void;
    static DrawSphere(center: Vector3, radius: number, depthTest: boolean): void;
    static DrawWireSphere(center: Vector3, radius: number, depthTest: boolean): void;
    static DrawCone(v1: Vector3, v2: Vector3, radius: number): void;
    static DrawIcon(center: Vector3, texture: Texture2D, allowScaling?: boolean, tint?: Color): void;
    private static set_color_impl;
    private static get_color_impl;
    private static set_matrix_impl;
    private static get_matrix_impl;
    private static clean_matrix_impl;
    private static set_line_width_impl;
    private static get_line_width_impl;
    private static draw_cube_2_impl;
    private static draw_wire_cube_impl;
    private static draw_line_impl;
    private static draw_line_color_impl;
    private static draw_sphere_impl;
    private static draw_wire_sphere_impl;
    private static draw_cone_impl;
    private static draw_icon_impl;
}
declare class MeshMerger {
    private static mesh_list;
    private static count;
    private static matrix_array;
    private static array_cout;
    static Merge(): Mesh;
    static Add(matrix: Matrix4x4, mesh: Mesh): void;
    static Clear(): void;
    private static merge_mesh;
}
declare class BehaviorRepeater extends BehaviorDecorator {
    constructor(handle?: never);
    get count(): BehaviorInt;
    get repeatForever(): BehaviorBool;
    get endOnFailure(): BehaviorBool;
    set count(val: BehaviorInt);
    set repeatForever(val: BehaviorBool);
    set endOnFailure(val: BehaviorBool);
    SetCount(count: number): void;
    SetRepeatForever(val: boolean): void;
    SetEndOnFailure(val: boolean): void;
    private static get_count;
    private static get_repeat_forever;
    private static get_end_on_failure;
    private static set_count;
    private static set_repeat_forever;
    private static set_end_on_failure;
    private static set_count_var;
    private static set_repeat_forever_var;
    private static set_end_on_failure_var;
}
declare class RenderQuality {
    static get isScreenShadow(): boolean;
    static get shadowDistance(): number;
    static get shadowCascadeType(): CascadeShadowType;
    static get shadowResolution(): ShadowResolutionType;
    static get shadowWeights(): number[];
    static get additionalLightShadowMapMaxResolution(): ShadowResolutionType;
    static get additionalLightSliceShadowMapResolution(): AdditionalLightSliceResolutionType;
    static get shaderLevel(): ShaderLevel;
    static get drawShadow(): boolean;
    static set shadowCascadeType(value: CascadeShadowType);
    static set shadowResolution(value: ShadowResolutionType);
    static set shadowDistance(value: number);
    static set additionalLightShadowMapMaxResolution(value: ShadowResolutionType);
    static set additionalLightSliceShadowMapResolution(value: AdditionalLightSliceResolutionType);
    static set shaderLevel(value: ShaderLevel);
    static set drawShadow(value: boolean);
    private static getIsScreenShadow;
    private static getShadowDistance;
    private static getShadowCascadeType;
    private static getShadowResolution;
    private static getShadowWeights;
    private static getAdditionalLightShadowMapMaxResolution;
    private static getAdditionalLightSliceShadowMapResolution;
    private static getShaderLevel;
    private static getDrawShadow;
    private static setShadowCascadeType;
    private static setShadowResolution;
    private static setShadowDistance;
    private static setAdditionalLightShadowMapMaxResolution;
    private static setAdditionalLightSliceShadowMapResolution;
    private static setShaderLevel;
    private static setDrawShadow;
}
declare class GUIPrefab extends AssetObject {
    private constructor();
    private static handle_check;
}
declare class Random {
    private _handle;
    constructor(handle?: never);
    SetSeed(value: number): void;
    NextInt(): number;
    NextIntRanged(min: number, max: number): number;
    NextFloat(): number;
    NextFloatRanged(min: number, max: number): number;
    RandomUnitVector(): Vector3;
    static SetSeed(value: number): void;
    static NextInt(): number;
    static NextIntRanged(min: number, max: number): number;
    static NextFloat(): number;
    static NextFloatRanged(min: number, max: number): number;
    static RandomUnitVector(): Vector3;
    static InsideUnitCircle(): Vector2;
    private static alloc_object;
    private static handle_check;
    private static set_seed_impl;
    private static next_impl;
    private static next_ranged_impl;
    private static next_float_impl;
    private static next_float_ranged_impl;
    private static next_unit_vector;
    private static st_next_ranged_impl;
    private static st_next_float_ranged_impl;
    private static st_next_unit_vector;
}
interface IHashCodeProvider {
    GetHashCode(): number;
}
declare class BehaviorUntilSuccess extends BehaviorDecorator {
    constructor(handle?: never);
}
declare namespace EDITOR {
    class GenerateTsCode {
        private static insertStr;
        private static findLinePosBegin;
        private static findMatchingBrace;
        private static findEndLinePosOfFunction;
        private static addPrivateMember;
        private static addUICanvasAssignment;
        private static generateCanvasCode;
        private static getObjectPath;
        private static getObjectName;
        static countNewlinesUpToIndex(text: string, index: number): number;
        private static addMemberDefine;
        private static addEvent;
        private static traverseAllChildren;
        private static ensureFunctionExists;
        private static ReadOrNewCode;
        private static getParentList;
        private static __GenerateAllObjectDefineCode;
        private static __GenerateTsDefineCode;
        private static __GenerateTsEventCode;
        private static RemovePrivateMemberCode;
        private static RemoveFindChildDefineCode;
        static RemoveAddEventCode(raw_code: string, ctrl: Control, objName: string): string;
        private static __RemoveGenerateTsCode;
    }
}
declare class RenderSettings {
    static get skybox(): Material;
    static get ambientColor(): Color;
    static get ambientGroundColor(): Color;
    static get ambientEquatorColor(): Color;
    static get ambientSkyColor(): Color;
    static get ambientMode(): AmbientMode;
    static get fogEnable(): boolean;
    static get fogDensity(): number;
    static get fogColor(): Color;
    static get fogMode(): FogMode;
    static get fogStartDistance(): number;
    static get fogEndDistance(): number;
    static get lightmapBlendAmbientFactor(): number;
    static set ambientColor(ambient_color: Color);
    static set procedualSkyboxSkyColor(sky_color: Color);
    static set procedualSkyboxGroundColor(ground_color: Color);
    static set ambientGroundColor(ambient_ground_color: Color);
    static set ambientEquatorColor(ambient_equator_color: Color);
    static set fogDensity(fog_density: number);
    static set fogStartDistance(fog_start: number);
    static set fogEndDistance(fog_end: number);
    static set fogColor(fog_color: Color);
    static set fogMode(fog_mode: FogMode);
    static set fogEnable(fog_enable: boolean);
    static set lightmapBlendAmbientFactor(factor: number);
    static SetSkybox(sbt: SkyboxType, skybox_mat: Material): void;
    private static getSkybox;
    private static getAmbientColor;
    private static getAmbientGroundColor;
    private static getAmbientEquatorColor;
    private static getAmbientSkyColor;
    private static getAmbientMode;
    private static getFogDensity;
    private static getFogColor;
    private static getFogMode;
    private static getFogEndDistance;
    private static getFogStartDistance;
    private static getFog;
    private static getLightmapBlendAmbientFactor;
    private static setSkybox_impl;
    private static setProcedualSkyboxSkyColor;
    private static setProcedualSkyboxGroundColor;
    private static setAmbientColor;
    private static setAmbientGroundColor;
    private static setAmbientEquatorColor;
    private static setFogDensity;
    private static setFogColor;
    private static setFogMode;
    private static setFogEndDistance;
    private static setFogStartDistance;
    private static setFog;
    private static setLightmapBlendAmbientFactor;
}
interface IOnResourceLoaded {
    (loaded_object: AssetObject): void;
}
interface IOnSpeechRecognizeCallBack {
    (type: number, resultJson: string): void;
}
declare class Material extends AssetObject {
    constructor(shader: Shader, handle?: never);
    get shader(): Shader;
    get renderQueue(): number;
    set renderQueue(value: number);
    set shader(value: Shader);
    GetFloat(name: string): number | undefined;
    GetVector(name: string): Vector4 | undefined;
    GetColor(name: string): Color | undefined;
    GetMatrix4x4(name: string): Matrix4x4 | undefined;
    GetTexture(name: string): Texture | null;
    HasFloat(name: string): boolean;
    HasVector(name: string): boolean;
    HasColor(name: string): boolean;
    HasMatrix4x4(name: string): boolean;
    HasTexture(name: string): boolean;
    SetFloat(name: string, value: number): void;
    SetVector(name: string, value: Vector4): void;
    SetColor(name: string, value: Color): void;
    SetMatrix4x4(name: string, value: Matrix4x4): void;
    SetTexture(name: string, tex: Texture): void;
    IsKeywordEnabled(value: string): boolean;
    DisableKeyword(value: string): void;
    EnableKeyword(value: string): void;
    private static alloc;
    private static handle_check;
    private static getRenderQueue_impl;
    private static setRenderQueue_impl;
    private static GetFloat_impl;
    private static GetVector_impl;
    private static GetColor_impl;
    private static GetMatrix4x4_impl;
    private static GetTexture_impl;
    private static HasFloat_impl;
    private static HasVector_impl;
    private static HasColor_impl;
    private static HasMatrix4x4_impl;
    private static HasTexture_impl;
    private static SetFloat_impl;
    private static SetVector_impl;
    private static SetColor_impl;
    private static SetMatrix4x4_impl;
    private static SetTexture_impl;
    private static IsKeywordEnabled_impl;
    private static DisableKeyword_impl;
    private static EnableKeyword_impl;
    private static SetShader_impl;
    private static GetShader_impl;
}
declare class BehaviorParallelComplete extends BehaviorComposite {
    constructor(handle?: never);
}
declare class ClosestPoint {
    outPos: Vector3;
    sprDist: number;
}
declare class BehaviorReturnFailure extends BehaviorDecorator {
    constructor(handle?: never);
}
declare class Texture2D extends Texture {
    constructor(handle: never);
    GetPixel(x: number, y: number, mipLevel: number): Color;
    GetPixels(x: number, y: number, blockWidth: number, blockHeight: number, mipLevel?: number): Color[] | undefined;
    SetPixel(x: number, y: number, col: Color, mipLevel: number): void;
    SetPixels(x: number, y: number, blockWidth: number, blockHeight: number, colors: Color[], mipLevel: number): void;
    Apply(mipLevel: number): void;
    ReleaseCPUMemory(): void;
    private static handle_check;
    private static get_pixel_impl;
    private static get_pixels_impl;
    private static set_pixel_impl;
    private static set_pixels_impl;
    private static apply_impl;
    private static release_cpu_memory_impl;
}
declare class ValueHashSet<T extends Number | String | Boolean> {
    private _handle;
    get count(): number;
    Add(item: Traits_PrimitiveType<T>): void;
    Remove(item: Traits_PrimitiveType<T>): void;
    Contains(item: Traits_PrimitiveType<T>): boolean;
    Clear(): void;
    toString(): string;
    forEach(callback: (item: Traits_PrimitiveType<T>) => void): void;
    [Symbol.iterator](): Generator<Traits_PrimitiveType<T>, void, unknown>;
}
declare class Application {
    static get mainViewport(): RenderViewport;
    static get FPS(): number;
    static set mainViewport(viewport: RenderViewport);
    static set FPS(val: number);
    static RemoveViewport(viewport: RenderViewport): void;
    private static getMainViewport;
    private static SetFPS;
    private static GetFPS;
    private static SetMainViewport_impl;
    private static RemoveViewport_impl;
}
declare class DataLibrary {
    private _handle;
    constructor(handle: never);
    GetData(file_name: string): string;
    SetData(file_name: string, data: string): void;
    GetDataWithMemoryStream(file_name: string): MemoryStream;
    SetDataWithMemoryStream(file_name: string, ms: MemoryStream): void;
    FileExists(file_name: string): boolean;
    RemoveFile(file_name: string): void;
    ClearFiles(): void;
    static get projectLibrary(): DataLibrary;
    private static handle_check;
    private static get_data_impl;
    private static set_data_impl;
    private static get_data_with_ms_impl;
    private static set_data_with_ms_impl;
    private static file_exists_impl;
    private static remove_file_impl;
    private static clear_files_impl;
    private static GetProjectLibrary_impl;
}
declare class BehaviorFloat extends BehaviorVariable {
    private constructor();
    GetValue(): number;
    SetValue(val: number): void;
    static Create(): BehaviorFloat;
    private static set_val;
    private static get_val;
}
declare class File {
    static Exists(path: string): boolean;
    static Delete(path: string): void;
    static Move(src: string, dst: string): void;
    static Copy(src: string, dst: string): void;
    static GetParent(path: string): string;
    static Create(path: string): void;
    static ReadAllBytes(path: string): MemoryStream | undefined;
    static ReadAllText(path: string): string;
    static WriteAllBytes(path: string, data: MemoryStream): boolean;
    static WriteAllText(path: string, content: string): void;
    private static exists_impl;
    private static delete_impl;
    private static move_impl;
    private static copy_impl;
    private static get_parent_impl;
    private static create_impl;
    private static read_all_bytes_impl;
    private static read_all_text_impl;
    private static write_all_bytes_impl;
    private static write_all_text_impl;
}
declare class Canvas extends Control {
    constructor(handle: never);
    get sortOrder(): number;
    get ppu(): number;
    get occludedRayMask(): number;
    set occludedRayMask(mask: number);
    AddEvent(event: ControlBaseEvent | ControlEvent, callBack: IControlEventCallBack | IControlEventCallBackI | IControlEventCallBackS): void;
    ClearEvent(event: ControlBaseEvent | ControlEvent, callBack?: IControlEventCallBack | IControlEventCallBackI | IControlEventCallBackS | null): void;
    private static handle_check;
    private static get_sort_order;
    private static get_ppu;
    private static get_3d_occluded_ray_mask;
    private static set_sort_order;
    private static set_3d_occluded_ray_mask;
}
declare class Light extends Component {
    constructor(handle: never);
    get type(): LightType;
    get color(): Color;
    get intensity(): number;
    get range(): number;
    get innerAngle(): number;
    get outerAngle(): number;
    get shadowType(): ShadowType;
    get shadowStrength(): number;
    get shadowCastLayerMask(): number;
    get shadowBias(): number;
    get shadowNormalBias(): number;
    set type(value: LightType);
    set color(value: Color);
    set intensity(value: number);
    set range(value: number);
    set innerAngle(value: number);
    set outerAngle(value: number);
    set shadowType(value: ShadowType);
    set shadowStrength(value: number);
    set cullingMask(value: number);
    set shadowBias(value: number);
    set shadowNormalBias(value: number);
    private static handle_check;
    private static getType_impl;
    private static getColor_impl;
    private static getIntensity_impl;
    private static getRange_impl;
    private static getInnerAngle_impl;
    private static getOuterAngle_impl;
    private static getShadowType_impl;
    private static getShadowStrength_impl;
    private static getCullingMask_impl;
    private static getShadowBias_impl;
    private static getShadowNormalBias_impl;
    private static setType_impl;
    private static setColor_impl;
    private static setIntensity_impl;
    private static setRange_impl;
    private static setInnerAngle_impl;
    private static setOuterAngle_impl;
    private static setShadowType_impl;
    private static setShadowStrength_impl;
    private static setCullingMask_impl;
    private static setShadowBias_impl;
    private static setShadowNormalBias_impl;
}
declare class BehaviorGameObject extends BehaviorVariable {
    private constructor();
    GetValue(): GameObject;
    SetValue(val: GameObject): void;
    static Create(): BehaviorGameObject;
    private static set_val;
    private static get_val;
}
declare class Button extends TransitionControl {
    constructor(handle: never);
    get bg(): Control;
    get label(): Control;
    get text(): string;
    set text(text: string);
    AddEvent(event: ButtonEvent | ControlBaseEvent | ControlEvent, callBack: IControlEventCallBack | IControlEventCallBackI | IControlEventCallBackS): void;
    ClearEvent(event: ButtonEvent | ControlBaseEvent | ControlEvent, callBack?: IControlEventCallBack | IControlEventCallBackI | IControlEventCallBackS | null): void;
    private static handle_check;
    private static get_bg_control;
    private static get_label_control;
    private static get_text;
    private static set_text;
}
declare class DynamicBonePlaneCollider extends Component {
    constructor(handle: never);
    private static handle_check;
}
declare class BehaviorGlobalVariable {
    private constructor();
    static AddVariable<T extends BehaviorVariable>(name: string, type: Traits_Constructor<T>): T | null;
    static RemoveVariable(name: string): void;
    static GetVariable(name: string): BehaviorVariable;
    static GetVariableList(): BehaviorVariable[];
    static Clear(): void;
    private static add_var;
    private static remove_var;
    private static get_var;
    private static get_var_list;
    private static clear_impl;
}
declare class BehaviorInt extends BehaviorVariable {
    private constructor();
    GetValue(): number;
    SetValue(val: number): void;
    static Create(name?: string): BehaviorInt;
    private static set_val;
    private static get_val;
}
declare class Text extends Control {
    constructor(handle: never);
    get text(): string;
    get material(): Material | null;
    get raycastTarget(): boolean;
    get transferred(): boolean;
    get richText(): boolean;
    get bestFit(): boolean;
    get horizontalOverflow(): boolean;
    get verticalOverflow(): boolean;
    get fontSize(): number;
    get minSize(): number;
    get maxSize(): number;
    get outline(): number;
    get lineSpacing(): Vector2;
    get offset(): Vector2;
    get color(): Color;
    get backgroundTexture(): Texture | null;
    get backgroundColor(): Color;
    get outlineColor(): Color;
    get fontName(): string;
    get fontStyle(): number;
    get hAlignment(): TextHAlignment;
    get vAlignment(): TextVAlignment;
    get hFitter(): TextFitter;
    get vFitter(): TextFitter;
    get contentSize(): Vector2;
    get lineSizeList(): Vector2[];
    set font(font: Font | null);
    set text(text: string);
    set material(material: Material | null);
    set raycastTarget(enable: boolean);
    set transferred(enable: boolean);
    set richText(enable: boolean);
    set bestFit(enable: boolean);
    set horizontalOverflow(enable: boolean);
    set verticalOverflow(enable: boolean);
    set fontSize(size: number);
    set minSize(size: number);
    set maxSize(size: number);
    set outline(outline: number);
    set lineSpacing(spacing: Vector2);
    set offset(offset: Vector2);
    set color(color: Color);
    set backgroundTexture(texture: Texture);
    set backgroundColor(color: Color);
    set outlineColor(color: Color);
    set fontStyle(style: number);
    set hAlignment(alignment: TextHAlignment);
    set vAlignment(alignment: TextVAlignment);
    set hFitter(fitter: TextFitter);
    set vFitter(fitter: TextFitter);
    UpdateContentSize(): void;
    AddEvent(event: TextEvent | ControlBaseEvent | ControlEvent, callBack: IControlEventCallBack | IControlEventCallBackI | IControlEventCallBackS): void;
    ClearEvent(event: TextEvent | ControlBaseEvent | ControlEvent, callBack?: IControlEventCallBack | IControlEventCallBackI | IControlEventCallBackS | null): void;
    private static handle_check;
    private static get_text;
    private static get_material;
    private static get_raycast_target;
    private static get_transferred;
    private static get_is_rich_text;
    private static get_best_fit;
    private static get_horizontal_overflow;
    private static get_vertical_overflow;
    private static get_font_size;
    private static get_min_size;
    private static get_max_size;
    private static get_outline;
    private static get_line_spacing;
    private static get_offset;
    private static get_color;
    private static get_background_texture;
    private static get_background_color;
    private static get_outline_color;
    private static get_font_name;
    private static get_font_style;
    private static get_h_alignment;
    private static get_v_alignment;
    private static get_h_fitter;
    private static get_v_fitter;
    private static get_content_size;
    private static get_line_size_list;
    private static set_font;
    private static set_text;
    private static set_material;
    private static set_raycast_target;
    private static set_transferred;
    private static set_is_rich_text;
    private static set_best_fit;
    private static set_horizontal_overflow;
    private static set_vertical_overflow;
    private static set_font_size;
    private static set_min_size;
    private static set_max_size;
    private static set_outline;
    private static set_line_spacing;
    private static set_offset;
    private static set_color;
    private static set_background_texture;
    private static set_background_color;
    private static set_outline_color;
    private static set_font_style;
    private static set_h_alignment;
    private static set_v_alignment;
    private static set_h_fitter;
    private static set_v_fitter;
    private static update_content_size;
}
declare class Default extends Control {
    constructor(handle: never);
    AddEvent(event: ControlBaseEvent | ControlEvent, callBack: IControlEventCallBack | IControlEventCallBackI | IControlEventCallBackS): void;
    ClearEvent(event: ControlBaseEvent | ControlEvent, callBack?: IControlEventCallBack | IControlEventCallBackI | IControlEventCallBackS | null): void;
    private static handle_check;
}
declare class BehaviorString extends BehaviorVariable {
    private constructor();
    GetValue(): string;
    SetValue(val: string): void;
    static Create(): BehaviorString;
    private static set_val;
    private static get_val;
}
declare class FileStream {
    private _handle;
    constructor(path: string, mode: FileMode, access?: FileAccess);
    get pos(): number;
    get length(): number;
    private _CheckHandle;
    WriteBytes(stream: MemoryStream): void;
    ReadBytes(stream: MemoryStream, count?: number): void;
    ReadAllString(): string;
    WriteAllString(val: string): void;
    Flush(): void;
    Seek(offset: number, dir?: FileDirection): void;
    Close(): void;
    private static alloc;
    private static alloc_with_access;
    private static get_pos_impl;
    private static get_length_impl;
    private static write_bytes_impl;
    private static read_bytes_impl;
    private static write_all_string;
    private static read_all_string;
    private static flush_impl;
    private static seek_impl;
    private static close_impl;
}
declare class MemoryStream {
    private _handle;
    private _max_len;
    private _buffer;
    private _buffer_view;
    private _buffer_offset;
    private _buffer_len;
    private _protect;
    private static littleEndian;
    private static _string_buffer;
    constructor(handle?: never);
    private check;
    private stringToBytes;
    private byteToString;
    get eof(): boolean;
    get pos(): number;
    get length(): number;
    get capacity(): number;
    set pos(p: number);
    set length(len: number);
    SetBool(val: boolean): void;
    SetSByte(val: number): void;
    SetShort(val: number): void;
    SetInt(val: number): void;
    SetLong(val: number): void;
    SetByte(val: number): void;
    SetUShort(val: number): void;
    SetUInt(val: number): void;
    SetFloat(val: number): void;
    SetDouble(val: number): void;
    SetVector2(val: Vector2): void;
    SetVector3(val: Vector3): void;
    SetVector4(val: Vector4): void;
    SetQuaternion(val: Quaternion): void;
    SetMatrix4x4(val: Matrix4x4): void;
    SetBounds(val: Bounds): void;
    SetColor(val: Color): void;
    SetCurve(val: Curve): void;
    SetGradient(val: Gradient): void;
    SetString(val: string): void;
    SetBoolArray(val: boolean[]): void;
    SetSByteArray(val: number[]): void;
    SetShortArray(val: number[]): void;
    SetIntArray(val: number[]): void;
    SetLongArray(val: number[]): void;
    SetByteArray(val: number[]): void;
    SetUShortArray(val: number[]): void;
    SetUIntArray(val: number[]): void;
    SetFloatArray(val: number[]): void;
    SetDoubleArray(val: number[]): void;
    SetVector2Array(val: Vector2[]): void;
    SetVector3Array(val: Vector3[]): void;
    SetVector4Array(val: Vector4[]): void;
    SetQuaternionArray(val: Quaternion[]): void;
    SetMatrix4x4Array(val: Matrix4x4[]): void;
    SetBoundsArray(val: Bounds[]): void;
    SetColorArray(val: Color[]): void;
    SetCurveArray(val: Curve[]): void;
    SetGradientArray(val: Gradient[]): void;
    SetStringArray(val: string[]): void;
    private check_read_impl;
    GetBool(): boolean;
    GetSByte(): number;
    GetShort(): number;
    GetInt(): number;
    GetLong(): number;
    GetByte(): number;
    GetUShort(): number;
    GetUInt(): number;
    GetFloat(): number;
    GetDouble(): number;
    GetVector2(): Vector2;
    GetVector3(): Vector3;
    GetVector4(): Vector4;
    GetQuaternion(): Quaternion;
    GetMatrix4x4(): Matrix4x4;
    GetBounds(): Bounds;
    GetColor(): Color;
    GetCurve(): Curve;
    GetGradient(): Gradient;
    GetString(): string;
    GetBoolArray(): boolean[];
    GetSByteArray(): number[];
    GetShortArray(): number[];
    GetIntArray(): number[];
    GetLongArray(): number[];
    GetByteArray(): number[];
    GetUShortArray(): number[];
    GetUIntArray(): number[];
    GetFloatArray(): number[];
    GetDoubleArray(): number[];
    GetVector2Array(): Vector2[];
    GetVector3Array(): Vector3[];
    GetVector4Array(): Vector4[];
    GetQuaternionArray(): Quaternion[];
    GetMatrix4x4Array(): Matrix4x4[];
    GetBoundsArray(): Bounds[];
    GetColorArray(): Color[];
    GetCurveArray(): Curve[];
    GetGradientArray(): Gradient[];
    GetStringArray(): string[];
    Reset(): void;
    CopyFrom(source: MemoryStream, mode: CloneMode): void;
    EnsureCapacity(size: number): void;
    PrepareRead(limit_length?: number): void;
    PrepareWrite(): void;
    SaveFile(path: string): void;
    ReadFile(path: string): void;
    private static alloc_object;
    private static handle_check;
    private static set_integer_impl;
    private static get_integer_impl;
    private static get_curve_impl;
    private static get_gradient_impl;
    private static save_file_impl;
    private static read_file_impl;
    private static update_memorystream_impl;
    private alloc_memorystream_impl;
}
declare class Path {
    static NormalizePath(path: string): string;
    static ChangeExtension(path: string, ext: string): string;
    static GetDirectoryName(path: string): string;
    static GetExtension(path: string): string;
    static GetFullPath(path: string): string;
    static GetFileName(path: string): string;
    static GetFileNameWithoutExtension(path: string): string;
    static GetPathRoot(path: string): string;
    static GetTempPath(): string;
    static GetRandomFileName(): string;
    static GetTempFileName(): string;
    static HasExtension(path: string): boolean;
    static MatchExtension(path: string, ext: string, ignore_case?: boolean): boolean;
    static IsPathRooted(path: string): boolean;
    static IsRelative(path: string): boolean;
    static GetRelativePath(relTo: string, path: string): string;
    private static normalize_path_impl;
    private static change_extension_impl;
    private static get_directory_name_impl;
    private static get_extension_impl;
    private static get_full_path_impl;
    private static get_file_name_impl;
    private static get_file_name_without_extension_impl;
    private static get_path_root_impl;
    private static get_temp_path_impl;
    private static get_random_file_name_impl;
    private static get_temp_file_name_impl;
    private static has_extension_impl;
    private static match_extension_impl;
    private static is_path_rooted_impl;
    private static is_relative_impl;
    private static get_relative_path_impl;
}
declare class PathManager {
    static rootPath(): string;
    static assetPath(): string;
    static CalculateByProjectPath(relative_path: string): string;
    static CalculateByCachePath(relative_path: string): string;
    static CalculateRelativeProjectPath(abs_path: string): string | undefined;
    private static get_root_path_impl;
    private static get_asset_path_impl;
}
declare class Bounds {
    private _center;
    private _extents;
    constructor(cnt?: Vector3, ext?: Vector3);
    get min(): Vector3;
    get max(): Vector3;
    get center(): Vector3;
    get extents(): Vector3;
    get size(): Vector3;
    set min(val: Vector3);
    set max(val: Vector3);
    set center(val: Vector3);
    set extents(val: Vector3);
    set size(val: Vector3);
    SetMinMax(min: Vector3, max: Vector3): void;
    Overlaps(other: Bounds): boolean;
    EncapsulatePoint(point: Vector3): void;
    EncapsulateBounds(bounds: Bounds): void;
    toString(): string;
    CopyFrom(from: Bounds): void;
    EqualsTo(other: Bounds): boolean;
    static get zero(): Bounds;
    static get infinity(): Bounds;
    static Clone(val: Bounds): Bounds;
    static EqualsTo(lhs: Bounds, rhs: Bounds): boolean;
}
declare class Vignette extends PostEffectBase {
    constructor(handle?: never);
    get color(): Color;
    get center(): Vector2;
    get intensity(): number;
    get smoothness(): number;
    get rounded(): boolean;
    set color(val: Color);
    set center(val: Vector2);
    set intensity(val: number);
    set smoothness(val: number);
    set rounded(val: boolean);
    private static alloc_object_impl;
    private static handle_check;
    private static get_color_impl;
    private static get_center_impl;
    private static get_intensity_impl;
    private static get_smoothness_impl;
    private static get_rounded_impl;
    private static set_color_impl;
    private static set_center_impl;
    private static set_intensity_impl;
    private static set_smoothness_impl;
    private static set_rounded_impl;
}
declare class NetEntity extends Component {
    constructor(handle: never);
    get isLocalObject(): boolean;
    get clientID(): number;
    get entityID(): string;
    get prefabID(): string;
    get isEnableInterpolation(): boolean;
    get positionThreshold(): number;
    get scaleThreshold(): number;
    get rotationThreshold(): number;
    get syncMask(): NetEntitySyncMask;
    get userdata(): string;
    set isEnableInterpolation(enable: boolean);
    set positionThreshold(t: number);
    set scaleThreshold(t: number);
    set rotationThreshold(t: number);
    set syncMask(mask: NetEntitySyncMask);
    setAssignable(): void;
    setOnBeforeDestroy(callback: (entityid: string) => void): void;
    setOnUserdataChange(callback: (data: string) => void): void;
    setGoParent(instID: number, keep_world_pos?: boolean): void;
    setEnableSync(value: boolean): void;
    setUserdata(data: string): void;
    Teleport(pos?: Vector3, quat4?: Quaternion, scl?: Vector3): void;
    private static handle_check;
    private static get_is_local_object;
    private static get_client_ID;
    private static get_entity_ID;
    private static get_prefab_ID;
    private static get_is_enable_interpolation;
    private static get_position_threshold;
    private static get_scale_threshold;
    private static get_rotation_threshold;
    private static get_sync_mask;
    private static get_userdata;
    private static set_is_enable_interpolation;
    private static set_position_threshold;
    private static set_scale_threshold;
    private static set_rotation_threshold;
    private static set_sync_mask;
    private static set_assignable;
    private static set_go_parent;
    private static set_userdata;
    private static set_enable_sync;
    private static set_on_before_destroy;
    private static set_on_userdata_change;
    private static teleport_impl;
}
declare class CharacterController extends Collider {
    constructor(handle: never);
    get velocity(): Vector3;
    get isGrounded(): boolean;
    get collisionFlags(): CollisionFlags;
    get radius(): number;
    get height(): number;
    get center(): Vector3;
    get slopeLimit(): number;
    get stepOffset(): number;
    get skinWidth(): number;
    get minMoveDistance(): number;
    get detectCollisions(): boolean;
    get minPositionIters(): number;
    set radius(value: number);
    set height(value: number);
    set center(value: Vector3);
    set slopeLimit(value: number);
    set stepOffset(value: number);
    set skinWidth(value: number);
    set minMoveDistance(value: number);
    set detectCollisions(value: boolean);
    set minPositionIters(value: number);
    Move(value: Vector3): CollisionFlags;
    SimpleMove(value: Vector3): boolean;
    private static handle_check;
    private static getVelocity_impl;
    private static getIsGrounded_impl;
    private static getCollisionFlags_impl;
    private static getRadius_impl;
    private static getHeight_impl;
    private static getCenter_impl;
    private static getSlopeLimit_impl;
    private static getStepOffset_impl;
    private static getSkinWidth_impl;
    private static getMinMoveDistance_impl;
    private static getDetectCollisions_impl;
    private static getMinPositionIters_impl;
    private static setRadius_impl;
    private static setHeight_impl;
    private static setCenter_impl;
    private static setSlopeLimit_impl;
    private static setStepOffset_impl;
    private static setSkinWidth_impl;
    private static setMinMoveDistance_impl;
    private static setDetectCollisions_impl;
    private static setMinPositionIters_impl;
    private static Move_impl;
    private static SimpleMove_impl;
}
declare class Color {
    r: number;
    g: number;
    b: number;
    a: number;
    constructor(r?: number, g?: number, b?: number, a?: number);
    get inverse(): Color;
    Inverse(): void;
    SetFromHSV(h: number, s: number, v: number): void;
    toString(): string;
    CopyFrom(from: Color): void;
    Get(index: number): number;
    Set(index: number, value: number): void;
    Add(rhs: Color): Color;
    Sub(rhs: Color): Color;
    Mul(rhs: number): Color;
    AddAssign(rhs: Color): void;
    SubAssign(rhs: Color): void;
    MulAssign(rhs: number): void;
    EqualsTo(other: Color): boolean;
    static Clone(val: Color): Color;
    static EqualsTo(lhs: Color, rhs: Color): boolean;
    static get white(): Color;
    static get black(): Color;
    static get red(): Color;
    static get green(): Color;
    static get blue(): Color;
    static get yellow(): Color;
    static get cyan(): Color;
    static get magenta(): Color;
    static get grey(): Color;
    static get clear(): Color;
}
declare class AnimationClip extends AssetObject {
    private constructor();
    get length(): number;
    get frameRate(): number;
    get curveCount(): number;
    private static handle_check;
    private static getLength_impl;
    private static getFrameRate_impl;
    private static getCurveCount_impl;
}
declare class Curve {
    private _handle;
    private _sorted;
    constructor(handle?: never);
    get(index: number): KeyFrame;
    get frameCount(): number;
    set(index: number, val: KeyFrame): void;
    AddFrame(frame: KeyFrame): void;
    RemoveFrame(index: number): void;
    Evaluate(time: number): number;
    EnsureOrder(): void;
    Clear(): void;
    toString(): string;
    private static alloc_object;
    private static handle_check;
    private static evaluate_impl;
    private static frame_count_impl;
    private static add_frame_impl;
    private static remove_frame_impl;
    private static get_frame_impl;
    private static set_frame_impl;
    private static ensure_order_impl;
    private static clear_impl;
}
declare class Frustum {
    plane: Vector4[];
    toString(): string;
    CopyFrom(from: Frustum): void;
    EqualsTo(other: Frustum): boolean;
    static Clone(val: Frustum): Frustum;
    static EqualsTo(lhs: Frustum, rhs: Frustum): boolean;
}
declare class Gradient {
    private _handle;
    private _sorted;
    constructor(handle?: never);
    get colorFrameCount(): number;
    get alphaFrameCount(): number;
    get mode(): GradientBlendMode;
    set mode(mode: GradientBlendMode);
    AddColorFrame(frame: KeyFrameColorRGB): void;
    RemoveColorFrame(index: number): void;
    AddAlphaFrame(frame: KeyFrame): void;
    RemoveAlphaFrame(index: number): void;
    Evaluate(time: number): Color;
    GetColorFrame(index: number): KeyFrameColorRGB;
    SetColorFrame(index: number, val: KeyFrameColorRGB): void;
    GetAlphaFrame(index: number): KeyFrame;
    SetAlphaFrame(index: number, val: KeyFrame): void;
    EnsureOrder(): void;
    Clear(): void;
    toString(): string;
    private static alloc_object;
    private static handle_check;
    private static evaluate_impl;
    private static color_count_impl;
    private static alpha_count_impl;
    private static get_mode_impl;
    private static set_mode_impl;
    private static add_color_frame_impl;
    private static remove_color_frame_impl;
    private static add_alpha_frame_impl;
    private static remove_alpha_frame_impl;
    private static get_color_frame_impl;
    private static set_color_frame_impl;
    private static get_alpha_frame_impl;
    private static set_alpha_frame_impl;
    private static ensure_order_impl;
    private static clear_impl;
}
declare class OffMeshLinkData {
    private _activated;
    private _endPos;
    private _linkType;
    private _offMeshLink;
    private _startPos;
    private _valid;
    get activated(): boolean;
    get endPos(): Vector3;
    get linkType(): OffMeshLinkType;
    get offMeshLink(): OffMeshLink;
    get startPos(): Vector3;
    get valid(): boolean;
}
declare class VerticalLayout extends LayoutGroup {
    constructor(handle: never);
    get spacing(): number;
    get reverse(): boolean;
    set spacing(spacing: number);
    set reverse(enable: boolean);
    private static handle_check;
    private static get_spacing;
    private static get_reverse;
    private static set_spacing;
    private static set_reverse;
}
declare class BehaviorEntry extends BehaviorParentTask {
    constructor(handle?: never);
}
declare class Tween {
    static get timeScale(): number;
    static set timeScale(value: number);
    static Sequence(): SequenceOperator;
    static UIPosition(rectTransform: RectTransform, from: Vector2, to: Vector2, duration: number): TweenerOperator;
    static UIPositionX(rectTransform: RectTransform, from: number, to: number, duration: number): TweenerOperator;
    static UIPositionY(rectTransform: RectTransform, from: number, to: number, duration: number): TweenerOperator;
    static UILocalPosition(rectTransform: RectTransform, from: Vector2, to: Vector2, duration: number): TweenerOperator;
    static UILocalPositionX(rectTransform: RectTransform, from: number, to: number, duration: number): TweenerOperator;
    static UILocalPositionY(rectTransform: RectTransform, from: number, to: number, duration: number): TweenerOperator;
    static UILocalEuler(rectTransform: RectTransform, from: number, to: number, duration: number): TweenerOperator;
    static UILocalScale(rectTransform: RectTransform, from: Vector2, to: Vector2, duration: number): TweenerOperator;
    static UILocalScaleX(rectTransform: RectTransform, from: number, to: number, duration: number): TweenerOperator;
    static UILocalScaleY(rectTransform: RectTransform, from: number, to: number, duration: number): TweenerOperator;
    static UIColor(control: Image | Text, from: Color, to: Color, duration: number): TweenerOperator;
    static Position(transform: Transform, from: Vector3, to: Vector3, duration: number): TweenerOperator;
    static PositionX(transform: Transform, from: number, to: number, duration: number): TweenerOperator;
    static PositionY(transform: Transform, from: number, to: number, duration: number): TweenerOperator;
    static PositionZ(transform: Transform, from: number, to: number, duration: number): TweenerOperator;
    static LocalPosition(transform: Transform, from: Vector3, to: Vector3, duration: number): TweenerOperator;
    static LocalPositionX(transform: Transform, from: number, to: number, duration: number): TweenerOperator;
    static LocalPositionY(transform: Transform, from: number, to: number, duration: number): TweenerOperator;
    static LocalPositionZ(transform: Transform, from: number, to: number, duration: number): TweenerOperator;
    static Euler(transform: Transform, from: Vector3, to: Vector3, duration: number): TweenerOperator;
    static EulerX(transform: Transform, from: number, to: number, duration: number): TweenerOperator;
    static EulerY(transform: Transform, from: number, to: number, duration: number): TweenerOperator;
    static EulerZ(transform: Transform, from: number, to: number, duration: number): TweenerOperator;
    static LocalEuler(transform: Transform, from: Vector3, to: Vector3, duration: number): TweenerOperator;
    static LocalEulerX(transform: Transform, from: number, to: number, duration: number): TweenerOperator;
    static LocalEulerY(transform: Transform, from: number, to: number, duration: number): TweenerOperator;
    static LocalEulerZ(transform: Transform, from: number, to: number, duration: number): TweenerOperator;
    static LocalScale(transform: Transform, from: Vector3, to: Vector3, duration: number): TweenerOperator;
    static LocalScaleX(transform: Transform, from: number, to: number, duration: number): TweenerOperator;
    static LocalScaleY(transform: Transform, from: number, to: number, duration: number): TweenerOperator;
    static LocalScaleZ(transform: Transform, from: number, to: number, duration: number): TweenerOperator;
    static Color(renderer: Renderer, colorName: string, from: Color, to: Color, duration: number): TweenerOperator;
    private static set_time_scale;
    private static get_time_scale;
    private static sequence;
    private static tweener_rect_transform_world_pos;
    private static tweener_rect_transform_world_pos_x;
    private static tweener_rect_transform_world_pos_y;
    private static tweener_rect_transform_local_pos;
    private static tweener_rect_transform_local_pos_x;
    private static tweener_rect_transform_local_pos_y;
    private static tweener_rect_transform_local_euler;
    private static tweener_rect_transform_local_scale;
    private static tweener_rect_transform_local_scale_x;
    private static tweener_rect_transform_local_scale_y;
    private static tweener_control_color;
    private static tweener_transform_world_pos;
    private static tweener_transform_world_pos_x;
    private static tweener_transform_world_pos_y;
    private static tweener_transform_world_pos_z;
    private static tweener_transform_local_pos;
    private static tweener_transform_local_pos_x;
    private static tweener_transform_local_pos_y;
    private static tweener_transform_local_pos_z;
    private static tweener_transform_euler;
    private static tweener_transform_euler_x;
    private static tweener_transform_euler_y;
    private static tweener_transform_euler_z;
    private static tweener_transform_local_euler;
    private static tweener_transform_local_euler_x;
    private static tweener_transform_local_euler_y;
    private static tweener_transform_local_euler_z;
    private static tweener_transform_local_scale;
    private static tweener_transform_local_scale_x;
    private static tweener_transform_local_scale_y;
    private static tweener_transform_local_scale_z;
    private static tweener_renderer_color;
}
declare class ColorGrading extends PostEffectBase {
    constructor(handle?: never);
    get customTexture(): Texture2D;
    get blendFactor(): number;
    set customTexture(value: Texture2D);
    set bakeDirty(value: boolean);
    set blendFactor(value: number);
    private static alloc_object_impl;
    private static handle_check;
    private static getCustomTexture_impl;
    private static get_blendFactor_impl;
    private static setCustomTexture_impl;
    private static set_blendFactor_impl;
    private static setBakeDirty_impl;
}
declare class InternalBuffers {
    static float_slot_0: Float32Array;
    static float_slot_1: Float32Array;
    static float_slot_2: Float32Array;
    static float_slot_3: Float32Array;
    static int_slot_0: Int32Array;
    static int_slot_1: Int32Array;
    static int_slot_2: Int32Array;
    static int_slot_3: Int32Array;
    static engine_shared_stream: MemoryStream;
    static script_namespace_map: Map<string, string>;
}
declare class AnimationMask extends AssetObject {
    private constructor();
    GetMask(value: string): Boolean;
    GetMaskByPath(value: string): Boolean;
    SetMask(name: string, mask: Boolean): void;
    SetMaskByPath(path: string, mask: Boolean): void;
    private static handle_check;
    private static getMask_impl;
    private static getMaskByPath_impl;
    private static setMask_impl;
    private static setMaskByPath_impl;
}
declare class KeyFrame {
    time: number;
    value: number;
    inTangent: number;
    outTangent: number;
    toString(): string;
    EqualsTo(other: KeyFrame): boolean;
    static EqualsTo(lhs: KeyFrame, rhs: KeyFrame): boolean;
}
declare class AnimatorData extends AssetObject {
    private constructor();
    private static handle_check;
}
declare class BehaviorIdle extends BehaviorAction {
    constructor(handle?: never);
}
declare class KeyFrameColorRGB {
    r: number;
    g: number;
    b: number;
    time: number;
    toString(): string;
    EqualsTo(other: KeyFrameColorRGB): boolean;
    static Lerp(lhs: KeyFrameColorRGB, rhs: KeyFrameColorRGB, weight: number): Color;
    static EqualsTo(lhs: KeyFrameColorRGB, rhs: KeyFrameColorRGB): boolean;
}
declare class Atlas extends AssetObject {
    private constructor();
    GetTexture(uuid: string, out_rect: Rect): Texture | null;
    GetTexUUIDList(): string[];
    private static handle_check;
    private static get_texture_impl;
    private static get_tex_uuid_list_impl;
}
declare class Mathf {
    static readonly pi: number;
    static readonly half_pi: number;
    static readonly double_pi: number;
    static readonly inv_pi: number;
    static readonly inv_pi_double: number;
    static readonly angleToRad: number;
    static readonly radToAngle: number;
    static readonly eps: number;
    static readonly sqrEps: number;
    static readonly ln2: number;
    static readonly ln2sqrt: number;
    static Sqrt(val: number): number;
    static FloorToInt(value: number): number;
    static CeilToInt(value: number): number;
    static Pow(f: number, p: number): number;
    static Min(lhs: number, rhs: number): number;
    static Max(lhs: number, rhs: number): number;
    static Clamp(val: number, left: number, right: number): number;
    static Lerp(left: number, right: number, t: number): number;
    static LerpUnclamped(left: number, right: number, t: number): number;
    static Sin(rad: number): number;
    static Cos(rad: number): number;
    static ASin(val: number): number;
    static ACos(val: number): number;
    static Tan(rad: number): number;
    static ATan2(y: number, x: number): number;
    static Floor(val: number): number;
    static Round(val: number): number;
    static Exp(val: number): number;
    static Sign(val: number): number;
    static Log(val: number, new_base?: number): number;
    static Log10(val: number): number;
    static SmoothDamp(source: number, target: number, velocity: number, smooth_time: number, max_speed: number, delta_time: number): number[];
    static SmoothDampAngle(source: number, target: number, velocity: number, smooth_time: number, max_speed: number, delta_time: number): number[];
    static MoveTowards(source: number, target: number, max_delta: number): number;
    static MoveTowardsAngle(source: number, target: number, max_delta: number): number;
}
declare class MathStructSerializer {
    private static push_vector2;
    private static push_vector3;
    private static push_vector4;
    private static push_quaternion;
    private static push_rect;
    private static push_bounds;
    private static push_matrix4x4;
    private static push_ray;
    private static push_color;
    private static pop_vector2;
    private static pop_vector3;
    private static pop_vector4;
    private static pop_quaternion;
    private static pop_rect;
    private static pop_bounds;
    private static pop_matrix4x4;
    private static pop_ray;
    private static pop_color;
}
declare class LayoutItem {
    private _handle;
    private _controlItem;
    constructor(handle: never);
    get instanceId(): number;
    get index(): number;
    get id(): number;
    get jsonStr(): string | null;
    get size(): Vector2;
    private get control();
    get controlItem(): ControlItem;
    set index(index: number);
    set id(id: number);
    set jsonStr(jsonStr: string);
    set size(size: Vector2);
    private set controlItem(value);
    toString(): string;
    private static handle_check;
    private static get_instance_id;
    private static get_index;
    private static get_id;
    private static get_json_str;
    private static get_size;
    private static get_control;
    private static set_index;
    private static set_id;
    private static set_json_str;
    private static set_size;
}
declare class AudioClip extends AssetObject {
    private constructor();
    get channels(): number;
    get frequency(): number;
    get length(): number;
    get loadType(): AudioClipLoadType;
    get samples(): number;
    static Create(name: string, stream: MemoryStream): AudioClip;
    private static handle_check;
    private static get_channels_impl;
    private static get_frequency_impl;
    private static get_length_impl;
    private static get_load_type_impl;
    private static get_samples_impl;
    private static create_impl;
}
declare class NetClient {
    private static _on_receive_message;
    private static _on_receive_pay_message;
    private static _on_netentity_created;
    private static _on_netentity_destroyed;
    private static _sender;
    static get onReceiveMessage(): IClientReceive;
    static get isConnected(): boolean;
    static get autoReConnect(): boolean;
    static get userid(): number;
    static get reqId(): string;
    static get rttMs(): number;
    static get netRttMs(): number;
    static set onReceiveMessage(callback: IClientReceive);
    static set onReceivePayMessage(callback: IClientPayReceive);
    static set serverIP(ip: string);
    static set autoReConnect(bval: boolean);
    static set serverPort(port: number);
    static Init(): void;
    static Connect(): void;
    static Disconnect(): void;
    static CreateNewEntitySync(prefab_id: string, userdata: string): NetEntity;
    static CreateNewTRSEntitySync(prefab_id: string, userdata?: string, init_pos?: Vector3, init_rota?: Quaternion, init_scale?: Vector3, callback?: (entity: NetEntity) => void): NetEntity;
    static CreateNewTRSEntity(prefab_id: string, userdata?: string, init_pos?: Vector3, init_rota?: Quaternion, init_scale?: Vector3, callback?: (entity: NetEntity) => void): void;
    static CreateNewEntity(prefab_id: string, userdata: string, callback?: (entity: NetEntity) => void): void;
    static FindEntity(entityID: string): NetEntity;
    static InjectOnConnectEvent(e: (flag?: boolean) => void): void;
    static InjectOnDisconnectEvent(e: (flag?: number) => void): void;
    static SetOnNetentityCreated(handler: (entity: NetEntity) => void): void;
    static SetOnNetentityDestroyed(handler: (entityID: string) => void): void;
    static ReqEntityCtrl(entity: NetEntity): void;
    static Send(message: string): void;
    static Emit<T extends Contract>(type: new (...args: any[]) => T, ...args: Parameters<T['Exescute']>): T;
    private static _OnContractReceive;
    private static get_is_connected;
    private static get_auto_reconnect;
    private static get_userid;
    private static get_reqid;
    private static get_rtt_ms;
    private static get_net_rtt_ms;
    private static set_auto_reconnect;
    private static set_on_receive_message;
    private static set_on_receive_pay_message;
    private static set_server_ip;
    private static set_server_port;
    private static init;
    private static conn_impl;
    private static dis_conn_impl;
    private static create_new_entity_impl;
    private static create_new_trs_entity_impl;
    private static create_new_trs_entity_sync_impl;
    private static create_new_entity_sync_impl;
    private static find_entity_impl;
    private static inject_connect_event;
    private static inject_disconnect_event;
    private static set_on_netentity_created;
    private static set_on_netentity_destroyed;
    private static req_entity_ctrl;
    private static send_impl;
}
declare class Matrix4x4 {
    m00: number;
    m10: number;
    m20: number;
    m30: number;
    m01: number;
    m11: number;
    m21: number;
    m31: number;
    m02: number;
    m12: number;
    m22: number;
    m32: number;
    m03: number;
    m13: number;
    m23: number;
    m33: number;
    get xAxis(): Vector3;
    get yAxis(): Vector3;
    get zAxis(): Vector3;
    get xScale(): number;
    get yScale(): number;
    get zScale(): number;
    get maxScale(): number;
    get position(): Vector3;
    get rotation(): Quaternion;
    get scale(): Vector3;
    get isPerspective(): boolean;
    get determinant(): number;
    get determinant_3x3(): number;
    get invert(): Matrix4x4;
    get transpose(): Matrix4x4;
    get invertTranspose(): Matrix4x4;
    SetColumn(index: number, column: Vector4): void;
    SetRow(index: number, column: Vector4): void;
    GetColumn(index: number): Vector4;
    GetRow(index: number): Vector4;
    SetIdentity(): void;
    Scale(scale: Vector3): void;
    PerspectiveMultiplyPoint3(v: Vector3): Vector3;
    MultiplyPoint(point: Vector3): Vector3;
    MultiplyPoint3x4(point: Vector3): Vector3;
    MultiplyVector(vector: Vector3): Vector3;
    MultiplyVector4(vector: Vector4): Vector4;
    toString(): string;
    CopyFrom(from: Matrix4x4): void;
    Get(index: number): number;
    Set(index: number, value: number): void;
    Add(rhs: Matrix4x4): Matrix4x4;
    Sub(rhs: Matrix4x4): Matrix4x4;
    Mul(rhs: Matrix4x4): Matrix4x4;
    AddAssign(rhs: Matrix4x4): void;
    SubAssign(rhs: Matrix4x4): void;
    MulAssign(rhs: Matrix4x4): void;
    EqualsTo(other: Matrix4x4): boolean;
    Invert(): void;
    Transpose(): void;
    InvertTranspose(): void;
    static FromTRS(pos: Vector3, rot: Quaternion, scale: Vector3): Matrix4x4;
    static FromTR(pos: Vector3, rot: Quaternion): Matrix4x4;
    static FromTranslate(pos: Vector3): Matrix4x4;
    static FromRotate(rot: Quaternion): Matrix4x4;
    static FromScale(scale: Vector3): Matrix4x4;
    static EqualsTo(lhs: Matrix4x4, rhs: Matrix4x4): boolean;
    static get zero(): Matrix4x4;
    static get identity(): Matrix4x4;
    private static GetRotImpl;
    private static InvertImpl;
    private static InvertTransposeImpl;
    private static TRSImpl;
    private static TRImpl;
    private static RImpl;
}
declare class Quaternion {
    x: number;
    y: number;
    z: number;
    w: number;
    constructor(x?: number, y?: number, z?: number, w?: number);
    get eulerAngles(): Vector3;
    get normalized(): Quaternion;
    get inverse(): Quaternion;
    get negative(): Quaternion;
    get xAxis(): Vector3;
    get yAxis(): Vector3;
    get zAxis(): Vector3;
    get almostZero(): boolean;
    SetValues(x: number, y: number, z: number, w: number): void;
    ToAxis(ref_xaxis: Vector3, ref_yaxis: Vector3, ref_zaxis: Vector3): void;
    ToAxisAngle(ref_axis: Vector3): number;
    Inverse(): void;
    Negative(): void;
    Normalize(): void;
    RotateVector(src: Vector3): Vector3;
    InvertRotateVector(src: Vector3): Vector3;
    toString(): string;
    CopyFrom(from: Quaternion): void;
    Get(index: number): number;
    Set(index: number, value: number): void;
    Add(rhs: Quaternion): Quaternion;
    Sub(rhs: Quaternion): Quaternion;
    Mul(rhs: Quaternion): Quaternion;
    AddAssign(rhs: Quaternion): void;
    SubAssign(rhs: Quaternion): void;
    MulAssign(rhs: Quaternion): void;
    EqualsTo(other: Quaternion): boolean;
    static get zero(): Quaternion;
    static get identity(): Quaternion;
    static Clone(val: Quaternion): Quaternion;
    static Dot(lhs: Quaternion, rhs: Quaternion): number;
    static Angle(lhs: Quaternion, rhs: Quaternion): number;
    static FromEuler(euler: Vector3): Quaternion;
    static FromEulerXYZ(x: number, y: number, z: number): Quaternion;
    static AxisToQuaternion(x: Vector3, y: Vector3, z: Vector3): Quaternion;
    static AngleAxis(angle: number, axis: Vector3): Quaternion;
    static LookRotation(forward: Vector3, up?: Vector3): Quaternion;
    static Lerp(q1: Quaternion, q2: Quaternion, t: number): Quaternion;
    static Slerp(q1: Quaternion, q2: Quaternion, t: number): Quaternion;
    static RotateTowards(from: Quaternion, to: Quaternion, max_delta_degrees: number): Quaternion;
    static FromToRotation(from_dir: Vector3, to_dir: Vector3): Quaternion;
    static EqualsTo(lhs: Quaternion, rhs: Quaternion): boolean;
    private static ToEulerImpl;
    private static FromEulderImpl;
    private static AxisToQuatImpl;
    private static AngleAxisImpl;
    private static LookRotationImpl;
    private static LerpImpl;
    private static SlerpImpl;
    private static FromToRotImpl;
}
declare class MeshRenderer extends Renderer {
    constructor(handle: never);
    get mesh(): Mesh | null;
    get sharedMesh(): Mesh | null;
    set mesh(value: Mesh);
    set sharedMesh(value: Mesh);
    private static handle_check;
    private static getMesh_impl;
    private static getSharedMesh_impl;
    private static setMesh_impl;
    private static setSharedMesh_impl;
}
declare class AnimationClipItem {
    clip: AnimationClip;
    setting: AnimationPlaySetting;
}
declare class Ray {
    private _origin;
    private _direction;
    constructor(origin?: Vector3, direction?: Vector3);
    get origin(): Vector3;
    get direction(): Vector3;
    GetRayDest(distance: number): Vector3;
    toString(): string;
    CopyFrom(from: Ray): void;
    EqualsTo(other: Ray): boolean;
    static Clone(val: Ray): Ray;
    static EqualsTo(lhs: Ray, rhs: Ray): boolean;
}
declare class Vector2 {
    x: number;
    y: number;
    constructor(x?: number, y?: number);
    get minChannel(): number;
    get maxChannel(): number;
    get magnitude(): number;
    get sqrMagnitude(): number;
    get inverse(): Vector2;
    get normalized(): Vector2;
    get negative(): Vector2;
    get abs(): Vector2;
    get almostZero(): boolean;
    SetValues(x: number, y: number): void;
    Scale(scale: Vector2): void;
    Negative(): void;
    Inverse(): void;
    Normalize(): void;
    toString(): string;
    CopyFrom(from: Vector2): void;
    Get(index: number): number;
    Set(index: number, value: number): void;
    Add(rhs: number | Vector2): Vector2;
    Sub(rhs: number | Vector2): Vector2;
    Mul(rhs: number | Vector2): Vector2;
    Div(rhs: number | Vector2): Vector2;
    AddAssign(rhs: number | Vector2): void;
    SubAssign(rhs: number | Vector2): void;
    MulAssign(rhs: number | Vector2): void;
    DivAssign(rhs: number | Vector2): void;
    EqualsTo(other: Vector2): boolean;
    static get zero(): Vector2;
    static get one(): Vector2;
    static get infinity(): Vector2;
    static Clone(val: Vector2): Vector2;
    static Lerp(lhs: Vector2, rhs: Vector2, t: number): Vector2;
    static LerpUnclamped(lhs: Vector2, rhs: Vector2, t: number): Vector2;
    static Dot(lhs: Vector2, rhs: Vector2): number;
    static Distance(lhs: Vector2, rhs: Vector2): number;
    static Min(lhs: Vector2, rhs: Vector2): Vector2;
    static Max(lhs: Vector2, rhs: Vector2): Vector2;
    static EqualsTo(lhs: Vector2, rhs: Vector2): boolean;
    static ClampMagnitude(source: Vector2, max_length: number): Vector2;
}
declare class Vector3 {
    x: number;
    y: number;
    z: number;
    constructor(x?: number, y?: number, z?: number);
    get minChannel(): number;
    get maxChannel(): number;
    get magnitude(): number;
    get sqrMagnitude(): number;
    get inverse(): Vector3;
    get normalized(): Vector3;
    get negative(): Vector3;
    get abs(): Vector3;
    get almostZero(): boolean;
    SetValues(x: number, y: number, z: number): void;
    Scale(scale: Vector3): void;
    Negative(): void;
    Inverse(): void;
    Normalize(): void;
    toString(): string;
    CopyFrom(from: Vector3): void;
    Get(index: number): number;
    Set(index: number, value: number): void;
    Add(rhs: number | Vector3): Vector3;
    Sub(rhs: number | Vector3): Vector3;
    Mul(rhs: number | Vector3): Vector3;
    Div(rhs: number | Vector3): Vector3;
    AddAssign(rhs: number | Vector3): void;
    SubAssign(rhs: number | Vector3): void;
    MulAssign(rhs: number | Vector3): void;
    DivAssign(rhs: number | Vector3): void;
    EqualsTo(other: Vector3): boolean;
    static get zero(): Vector3;
    static get one(): Vector3;
    static get up(): Vector3;
    static get down(): Vector3;
    static get left(): Vector3;
    static get right(): Vector3;
    static get forward(): Vector3;
    static get back(): Vector3;
    static get infinity(): Vector3;
    static Clone(val: Vector3): Vector3;
    static Lerp(lhs: Vector3, rhs: Vector3, t: number): Vector3;
    static LerpUnclamped(lhs: Vector3, rhs: Vector3, t: number): Vector3;
    static Cross(lhs: Vector3, rhs: Vector3): Vector3;
    static Dot(lhs: Vector3, rhs: Vector3): number;
    static Distance(lhs: Vector3, rhs: Vector3): number;
    static Min(lhs: Vector3, rhs: Vector3): Vector3;
    static Max(lhs: Vector3, rhs: Vector3): Vector3;
    static Angle(from: Vector3, to: Vector3): number;
    static MoveTowards(source: Vector3, target: Vector3, max_delta_dis: number): Vector3;
    static RotateTowards(source: Vector3, target: Vector3, max_delta_radians: number, max_delta_mag: number): Vector3;
    static EqualsTo(lhs: Vector3, rhs: Vector3): boolean;
    static ClampMagnitude(source: Vector3, max_length: number): Vector3;
    private static RotateTowardsImpl;
}
declare class Vector4 {
    x: number;
    y: number;
    z: number;
    w: number;
    constructor(x?: number, y?: number, z?: number, w?: number);
    get minChannel(): number;
    get maxChannel(): number;
    get magnitude(): number;
    get sqrMagnitude(): number;
    get inverse(): Vector4;
    get normalized(): Vector4;
    get negative(): Vector4;
    get abs(): Vector4;
    get almostZero(): boolean;
    SetValues(x: number, y: number, z: number, w: number): void;
    Scale(scale: Vector4): void;
    Negative(): void;
    Inverse(): void;
    Normalize(): void;
    toString(): string;
    CopyFrom(from: Vector4): void;
    Get(index: number): number;
    Set(index: number, value: number): void;
    Add(rhs: number | Vector4): Vector4;
    Sub(rhs: number | Vector4): Vector4;
    Mul(rhs: number | Vector4): Vector4;
    Div(rhs: number | Vector4): Vector4;
    AddAssign(rhs: number | Vector4): void;
    SubAssign(rhs: number | Vector4): void;
    MulAssign(rhs: number | Vector4): void;
    DivAssign(rhs: number | Vector4): void;
    EqualsTo(other: Vector4): boolean;
    static get zero(): Vector4;
    static get one(): Vector4;
    static get infinity(): Vector4;
    static Clone(val: Vector4): Vector4;
    static Lerp(lhs: Vector4, rhs: Vector4, t: number): Vector4;
    static LerpUnclamped(lhs: Vector4, rhs: Vector4, t: number): Vector4;
    static Dot(lhs: Vector4, rhs: Vector4): number;
    static Distance(lhs: Vector4, rhs: Vector4): number;
    static Min(lhs: Vector4, rhs: Vector4): Vector4;
    static Max(lhs: Vector4, rhs: Vector4): Vector4;
    static EqualsTo(lhs: Vector4, rhs: Vector4): boolean;
}
declare class AudioRecord {
    private static autoUpload;
    private static duration;
    private static eventOnAudioRecordCallback;
    static set OnAudioRecordCallback(callback: IOnAudioRecordCallBack);
    static Init(autoUpload?: boolean): void;
    static Release(): void;
    static StartAudioRecord(): void;
    static StopAudioRecord(): void;
    static CancelAudioRecord(): void;
    static PlayAudioRecord(path: string): void;
    private static audio_record_callback;
    private static upload_voice_file_result;
}
declare enum AudioRecordOperation {
    StartRecord = 1,
    StopRecord = 2,
    CancelRecord = 3,
    PlayVoive = 11,
    StopVoive = 12,
    PlayUrl = 21
}
declare class NetClientSender implements IContracter {
    FindEntity(entityID: string): NetEntity;
    CreateNewEntity(prefab_id: string, userdata: string, callback?: (entity: NetEntity) => void): void;
    CurrentConversation(): NetConversation;
}
declare class CurrencyOperationResult {
    success: boolean;
    current_amount: number;
}
declare class GetGlobalDataKeysResult {
    success: boolean;
    keys: string[];
    userids: string[];
}
declare class NetConversation extends EngineObject implements IContracter {
    private _handle;
    constructor(handle: never);
    get isNew(): boolean;
    get userID(): number;
    get clientID(): number;
    get platID(): number;
    get server_id(): string;
    Send(message: string): void;
    Emit<T extends Contract>(type: new (...args: any[]) => T, ...args: Parameters<T['Exescute']>): T;
    FindEntity(entityID: string): NetEntity;
    CreateNewEntity(prefab_id: string, userdata: string, callback?: (entity: NetEntity) => void): void;
    CurrentConversation(): NetConversation;
    ConnectToImpl(token: NetInstanceToken): boolean;
    ConnectTo(token: NetInstanceToken): boolean;
    NewUniqueID(type: string): Promise<number>;
    GetCurrencyAmount(type: string): Promise<number>;
    ChangeCurrencyAmount(type: string, diff: number): Promise<CurrencyOperationResult>;
    GetStoredValue(key: string): Promise<string>;
    SetStoredValue(key: string, value: string): Promise<void>;
    GetGlobalDataKeys(): Promise<GetGlobalDataKeysResult>;
    GetGlobalData(key: string, userid: number): Promise<GetGlobalDataResult>;
    SetGlobalData(key: string, val: string): Promise<boolean>;
    RequestBatchAddDress(suitIds: number[], decorationIds: number[]): Promise<boolean>;
    private static handle_check;
    private static get_isnew_impl;
    private static get_user_id_impl;
    private static get_client_id_impl;
    private static send_impl;
    private static find_entity_impl;
    private static connectimpl_to_impl;
    private static connect_to_impl;
    private static new_unique_id_impl;
    private static get_currency_amount_impl;
    private static change_currency_amount_impl;
    private static get_value_impl;
    private static set_value_impl;
    private static get_global_data_keys_impl;
    private static get_global_data_impl;
    private static set_global_data_impl;
    private static get_plat_id_impl;
    private static request_batch_add_dress_impl;
}
declare class GetGlobalDataResult {
    success: boolean;
    values: string;
}
declare class NetServer {
    private static _on_receive_message;
    private static _on_conv_connect;
    private static _on_conv_disconnect;
    private static _on_payorder_result;
    private static _on_get_goodslist;
    private static _sender;
    static get receive_message_handler(): (conv: NetConversation, msg: string) => void;
    static get on_conversation_connect(): (conv: NetConversation) => void;
    static get on_conversation_disconnect(): (conv: NetConversation) => void;
    static get on_payorder_result(): (conv: NetConversation, orderid: string, goodsid: number, count: number) => boolean;
    static get on_get_goodslist(): (goodsjson: string) => void;
    static get conversations(): NetConversation[];
    static get tag(): string;
    static get cmdparam(): string;
    static set receive_message_handler(handler: (conv: NetConversation, msg: string) => void);
    static set on_conversation_connect(handler: (conv: NetConversation) => void);
    static set on_conversation_disconnect(handler: (conv: NetConversation) => void);
    static set on_payorder_result(handler: (conv: NetConversation, orderid: string, goodsid: number, count: number) => boolean);
    static set on_get_goodslist(handler: (goodsjson: string) => void);
    static SetOnNetentityCreated(handler: (entity: NetEntity) => void): void;
    static SetOnNetentityDestroyed(handler: (entityID: string) => void): void;
    static SetReconnectionTimeout(timeout: number): void;
    static Managed(com: Component): boolean;
    static FindConversation(client_id: number): NetConversation;
    static CreateNewTRSEntity(prefab_id: string, userdata?: string, init_pos?: Vector3, init_rota?: Quaternion, init_scale?: Vector3, callback?: (entity: NetEntity) => void): void;
    static CreateNewEntity(prefab_id: string, userdata: string, callback?: (entity: NetEntity) => void): void;
    static FindEntity(entityID: string): NetEntity;
    static EmitAndBroadcast<T extends Contract>(type: new (...args: any[]) => T, ...args: Parameters<T['Exescute']>): void;
    static CreateInstance(scene_id: string, remote?: boolean): Promise<NetInstanceToken>;
    static FindInstance(server_id: string): Promise<NetInstanceToken>;
    static SendStatusChg(status: number): void;
    static RequestGameRankList(key: string, pageSize: number, offset: number, userId: number, type: number): Promise<{
        success: boolean;
        response: string;
    }>;
    static RequestReportGameRank(key: string, member: number, score: number, type: number): Promise<{
        success: boolean;
        response: string;
    }>;
    static RequestBatchGameRankList(userId: number, keys: string, type: number): Promise<{
        success: boolean;
        response: string;
    }>;
    private static _OnContractReceive;
    private static set_on_netentity_created;
    private static set_on_netentity_destroyed;
    private static get_conversations_impl;
    private static get_tag_impl;
    private static set_receive_hnd_impl;
    private static set_on_payorder_result_impl;
    private static set_on_get_goodslist_impl;
    private static set_on_conv_conn_impl;
    private static set_on_conv_disconn_impl;
    private static managed_component_impl;
    private static find_conv_impl;
    private static create_new_trs_entity_impl;
    private static create_new_entity_impl;
    private static find_entity_impl;
    private static create_instance_impl;
    private static find_instance_impl;
    private static set_reconnection_timeout_impl;
    private static get_cmdparam_impl;
    private static send_status_chg_impl;
    private static request_game_rank_list_impl;
    private static request_report_game_rank_impl;
    private static request_batch_game_rank_list_impl;
}
declare class BehaviorSequence extends BehaviorComposite {
    constructor(handle?: never);
}
declare class Form extends Control {
    constructor(handle: never);
    AddEvent(event: ControlBaseEvent | ControlEvent, callBack: IControlEventCallBack | IControlEventCallBackI | IControlEventCallBackS): void;
    ClearEvent(event: ControlBaseEvent | ControlEvent, callBack?: IControlEventCallBack | IControlEventCallBackI | IControlEventCallBackS | null): void;
    private static handle_check;
}
declare class CCDIK extends IKSolver {
    constructor(handle: never);
    get target(): Transform | null;
    set target(transform: Transform);
    private static handle_check;
    private static getTarget;
    private static setTarget;
}
declare class BehaviorLog extends BehaviorAction {
    constructor(handle?: never);
    get text(): BehaviorString;
    get logType(): BehaviorLogType;
    set text(val: BehaviorString);
    set logType(type: BehaviorLogType);
    SetText(text: string): void;
    private static set_text_var;
    private static set_text_str;
    private static set_log_type;
    private static get_text;
    private static get_log_type;
}
declare class GridLayout extends LayoutGroup {
    constructor(handle: never);
    get spacing(): Vector2;
    get fillStart(): LayoutFillStartType;
    get fillDirection(): LayoutFillDirectionType;
    get fillLimit(): LayoutFillLimitType;
    get fillLimitNum(): number;
    set spacing(spacing: Vector2);
    set fillStart(start: LayoutFillStartType);
    set fillDirection(direction: LayoutFillDirectionType);
    set fillLimit(limit: LayoutFillLimitType);
    set fillLimitNum(limitNum: number);
    private static handle_check;
    private static get_spacing;
    private static get_fill_start;
    private static get_fill_direction;
    private static get_fill_limit;
    private static get_fill_limit_num;
    private static set_spacing;
    private static set_fill_start;
    private static set_fill_direction;
    private static set_fill_limit;
    private static set_fill_limit_num;
}
declare class GUISpriteSequence extends Control {
    constructor(handle: never);
    get raycastTarget(): boolean;
    get raycastPadding(): Rect;
    get color(): Color;
    get spriteSequence(): SpriteSequence | null;
    get uv(): Rect;
    get loopCount(): number;
    get loop(): boolean;
    get isPlaying(): boolean;
    get isStop(): boolean;
    get isPause(): boolean;
    set raycastTarget(value: boolean);
    set raycastPadding(value: Rect);
    set color(value: Color);
    set spriteSequence(value: SpriteSequence | null);
    set loopCount(value: number);
    set loop(value: boolean);
    Play(): void;
    Stop(): void;
    Continue(): void;
    Pause(): void;
    AddEvent(event: GUISpriteSequenceEvent | ControlBaseEvent | ControlEvent, callBack: IControlEventCallBack | IControlEventCallBackI | IControlEventCallBackS): void;
    ClearEvent(event: GUISpriteSequenceEvent | ControlBaseEvent | ControlEvent, callBack?: IControlEventCallBack | IControlEventCallBackI | IControlEventCallBackS | null): void;
    private static handle_check;
    private static get_raycast_target;
    private static get_raycast_padding;
    private static get_color;
    private static get_sprite_sequence;
    private static get_uv;
    private static get_loop_count;
    private static is_loop;
    private static is_playing;
    private static is_stop;
    private static is_pause;
    private static set_raycast_target;
    private static set_raycast_padding;
    private static set_color;
    private static set_sprite_sequence;
    private static set_loop_count;
    private static set_loop;
    private static play_impl;
    private static stop_impl;
    private static continue_impl;
    private static pause_impl;
}
declare class AimIK extends IKSolver {
    constructor(handle: never);
    get fixTransform(): boolean;
    get aimTransform(): Transform | null;
    get poleTarget(): Transform | null;
    get target(): Transform | null;
    get axis(): Vector3;
    get poleAxis(): Vector3;
    get poleWeight(): number;
    set fixTransform(isFix: Boolean);
    set aimTransform(transform: Transform);
    set poleTarget(transform: Transform);
    set target(transform: Transform);
    set axis(aixs: Vector3);
    set poleAxis(poleAxis: Vector3);
    set poleWeight(poleWeight: number);
    private static handle_check;
    private static getFixTransform;
    private static getAimTransform;
    private static getPoleTarget;
    private static getTarget;
    private static getAxis;
    private static getPoleAxis;
    private static getPoleWeight;
    private static setFixTransform;
    private static setAimTransform;
    private static setPoleTarget;
    private static setTarget;
    private static setAxis;
    private static setPoleAxis;
    private static setPoleWeight;
}
declare class DelegateItem {
    func: any;
    func_model: any;
    obj: any;
}
declare class Animation extends Component {
    constructor(handle: never);
    get clip(): AnimationClip | null;
    get clipCount(): number;
    get isPlaying(): boolean;
    get localBounds(): Bounds;
    get playAutomatically(): boolean;
    get wrapMode(): WrapMode;
    set clip(clip: AnimationClip);
    set setting(setting: AnimationPlaySetting);
    set localBounds(val: Bounds);
    set playAutomatically(val: boolean);
    set wrapMode(mode: WrapMode);
    AddClip(clip: AnimationClip, name: string, setting?: AnimationPlaySetting): void;
    Blend(name: string, weight: number, normalized_duration: number, mode: AnimationPlayMode): void;
    CrossFade(name: string, normalized_duration: number): void;
    CrossFadeQueued(name: string, normalized_duration: number, queue_mode: AnimationQueueMode, mode: AnimationPlayMode): AnimationState;
    GetClip(name: string): AnimationClip;
    GetClips(): AnimationClipItem[];
    GetState(name: string): AnimationState;
    IsPlaying(name: string): boolean;
    Play(name?: string | null, mode?: AnimationPlayMode): boolean;
    PlayQueued(name: string, queue_mode: AnimationQueueMode, mode: AnimationPlayMode): void;
    RemoveClip(clip: AnimationClip): void;
    RemoveClipByName(name: string): void;
    Rewind(name: string): void;
    Sample(): void;
    SetClips(clips: AnimationClipItem[]): void;
    Stop(name?: string | null): void;
    SyncLayer(layer: number): void;
    private static handle_check;
    private static get_clip_impl;
    private static get_clip_count_impl;
    private static get_is_playing_impl;
    private static get_local_bounds_impl;
    private static get_play_automatically_impl;
    private static get_wrap_mode_impl;
    private static set_clip_impl;
    private static set_setting_impl;
    private static set_local_bounds_impl;
    private static set_play_automatically_impl;
    private static set_wrap_mode_impl;
    private static add_clip_impl;
    private static blend_impl;
    private static cross_fade_impl;
    private static cross_fade_queued_impl;
    private static get_clip_with_name_impl;
    private static get_clips_impl;
    private static get_state_impl;
    private static is_playing_impl;
    private static play_impl;
    private static play_queued_impl;
    private static remove_clip_name_impl;
    private static remove_clip_clip_impl;
    private static rewind_impl;
    private static sample_impl;
    private static set_clips_impl;
    private static stop_impl;
    private static sync_layer_impl;
}
declare class NetDatabase {
    static CreateDataGetter(): NetDBGetter;
    static CreateDataSetter(): NetDBSetter;
    static GetGlobalAmount(type: string): Promise<number>;
    static ChangeGlobalAmount(type: string, diff: number): Promise<CurrencyOperationResult>;
    private static create_db_getter;
    private static create_db_setter;
    private static get_currency_amount_impl;
    private static change_currency_amount_impl;
}
declare class AudioSource extends Component {
    constructor(handle: never);
    get time(): number;
    get timeSamples(): number;
    get clip(): AudioClip;
    get priority(): number;
    get isPlaying(): boolean;
    get isVirtual(): boolean;
    get loop(): boolean;
    get playOnAwake(): boolean;
    get mute(): boolean;
    get pitch(): number;
    get volume(): number;
    get spatialBlend(): number;
    get panStereo(): number;
    get minDistance(): number;
    get maxDistance(): number;
    get rolloffMode(): AudioRolloffMode;
    get rolloffCurve(): Vector2[];
    get dopplerLevel(): number;
    get spread(): number;
    get playState(): AudioPlayState;
    set time(val: number);
    set timeSamples(val: number);
    set clip(val: AudioClip);
    set priority(val: number);
    set loop(val: boolean);
    set playOnAwake(val: boolean);
    set mute(val: boolean);
    set pitch(val: number);
    set volume(val: number);
    set spatialBlend(val: number);
    set panStereo(val: number);
    set minDistance(val: number);
    set maxDistance(val: number);
    set rolloffMode(val: AudioRolloffMode);
    set rolloffCurve(curves: Vector2[]);
    set dopplerLevel(val: number);
    set spread(val: number);
    Play(): void;
    Pause(paused: boolean): void;
    Stop(): void;
    private static handle_check;
    private static get_time_impl;
    private static get_time_samples_impl;
    private static get_clip_impl;
    private static get_priority_impl;
    private static get_is_playing_impl;
    private static get_is_virtual_impl;
    private static get_loop_impl;
    private static get_play_on_awake_impl;
    private static get_mute_impl;
    private static get_pitch_impl;
    private static get_volume_impl;
    private static get_spatial_blend_impl;
    private static get_pan_stereo_impl;
    private static get_min_distance_impl;
    private static get_max_distance_impl;
    private static get_rolloff_mode_impl;
    private static get_rolloff_curve_impl;
    private static get_doppler_level_impl;
    private static get_spread_impl;
    private static set_time_impl;
    private static set_time_samples_impl;
    private static set_clip_impl;
    private static set_priority_impl;
    private static set_loop_impl;
    private static set_play_on_awake_impl;
    private static set_mute_impl;
    private static set_pitch_impl;
    private static set_volume_impl;
    private static set_spatial_blend_impl;
    private static set_pan_stereo_impl;
    private static set_min_distance_impl;
    private static set_max_distance_impl;
    private static set_rolloff_mode_impl;
    private static set_rolloff_curve_impl;
    private static set_doppler_level_impl;
    private static set_spread_impl;
    private static play_impl;
    private static pause_impl;
    private static unpause_impl;
    private static stop_impl;
    private static get_play_state_impl;
}
declare class BoxCollider extends Collider {
    constructor(handle: never);
    get center(): Vector3;
    get size(): Vector3;
    get extents(): Vector3;
    set center(center: Vector3);
    set size(size: Vector3);
    set extents(extents: Vector3);
    private static handle_check;
    private static getCenter;
    private static getSize;
    private static getExtents;
    private static setCenter;
    private static setSize;
    private static setExtents;
}
declare class Camera extends Component {
    constructor(handle: never);
    get depthTextureMode(): DepthTextureMode;
    get cullingMatrix(): Matrix4x4;
    get cameraType(): CameraType;
    get clearFlags(): CameraClearFlags;
    get clearColor(): Color;
    get aspect(): number;
    get depth(): number;
    get orthographic(): boolean;
    get orthographicSize(): number;
    get farClipPlane(): number;
    get nearClipPlane(): number;
    get fieldOfView(): number;
    get cullingMask(): number;
    get rect(): Rect;
    get projectionMatrix(): Matrix4x4;
    get worldToCameraMatrix(): Matrix4x4;
    get cameraToWorldMatrix(): Matrix4x4;
    get targetTexture(): RenderTexture;
    get requireOpaqueTexture(): boolean;
    get requireDepthTexture(): boolean;
    set depthTextureMode(mode: DepthTextureMode);
    set cullingMatrix(mtx: Matrix4x4);
    set cameraType(type: CameraType);
    set clearFlags(flag: CameraClearFlags);
    set clearColor(color: Color);
    set aspect(aspect: number);
    set depth(depth: number);
    set orthographic(is_ortho: boolean);
    set orthographicSize(ortho_size: number);
    set farClipPlane(far: number);
    set nearClipPlane(near: number);
    set fieldOfView(fov: number);
    set cullingMask(mask: number);
    set rect(rect: Rect);
    set projectionMatrix(mtx: Matrix4x4);
    set worldToCameraMatrix(mtx: Matrix4x4);
    set targetTexture(rt: RenderTexture);
    set requireOpaqueTexture(require: boolean);
    set requireDepthTexture(require: boolean);
    ScreenPointToRay(point: Vector3): Ray;
    ScreenPointToViewport(point: Vector3): Vector3;
    ScreenPointToWorld(point: Vector3): Vector3;
    ViewportPointToRay(point: Vector3): Ray;
    ViewportPointToScreen(point: Vector3): Vector3;
    ViewportPointToWorld(point: Vector3): Vector3;
    WorldPointToScreen(point: Vector3): Vector3;
    WorldPointToViewport(point: Vector3): Vector3;
    SaveTargetTexturePngToFile(filePath: string): void;
    static get allCameraCount(): number;
    static get allCameras(): Camera[];
    static get mainCamera(): Camera;
    static get currentCamera(): Camera;
    private static handle_check;
    private static get_dt_mode_impl;
    private static get_cull_matrix_impl;
    private static get_camrea_type_impl;
    private static get_clear_flags;
    private static get_clear_color_impl;
    private static get_aspect_impl;
    private static get_depth_impl;
    private static get_ortho_impl;
    private static get_ortho_size_impl;
    private static get_far_clip_impl;
    private static get_near_clip_impl;
    private static get_fov_impl;
    private static get_culling_mask;
    private static get_rect_impl;
    private static get_proj_impl;
    private static get_w2cm_impl;
    private static get_c2wm_impl;
    private static get_target_tex_impl;
    private static get_require_opaque_texture_impl;
    private static get_require_depth_texture_impl;
    private static set_dt_mode_impl;
    private static set_cull_matrix_impl;
    private static set_camrea_type_impl;
    private static set_clear_flags;
    private static set_clear_color_impl;
    private static set_aspect_impl;
    private static set_depth_impl;
    private static set_ortho_impl;
    private static set_ortho_size_impl;
    private static set_far_clip_impl;
    private static set_near_clip_impl;
    private static set_fov_impl;
    private static set_culling_mask;
    private static set_rect_impl;
    private static set_proj_impl;
    private static set_w2cm_impl;
    private static set_target_tex_impl;
    private static set_require_opaque_texture_impl;
    private static set_require_depth_texture_impl;
    private static screen_pos_to_ray_impl;
    private static screen_pos_to_vp_impl;
    private static screen_pos_to_wld_impl;
    private static vp_pos_to_ray_impl;
    private static vp_pos_to_screen_impl;
    private static vp_pos_to_world_impl;
    private static wld_pos_to_screen_impl;
    private static wld_pos_to_vp_impl;
    private static all_cam_cnt_impl;
    private static all_cam_impl;
    private static main_cam_impl;
    private static curr_cam_impl;
    private static save_target_texture_png_to_file;
}
declare class CapsuleCollider extends Collider {
    constructor(handle: never);
    get center(): Vector3;
    get radius(): number;
    get height(): number;
    get capsuleCollderDirection(): CapsuleCollderDirection;
    set center(center: Vector3);
    set radius(radius: number);
    set height(height: number);
    set capsuleCollderDirection(direction: CapsuleCollderDirection);
    private static handle_check;
    private static getCenter;
    private static getRadius;
    private static getHeight;
    private static getDirection;
    private static setCenter;
    private static setRadius;
    private static setHeight;
    private static setDirection;
}
declare class DynamicBone extends Component {
    constructor(handle: never);
    get root(): Transform | null;
    get updateRate(): number;
    get updateMode(): number;
    get damping(): number;
    get elasticity(): number;
    get stiffness(): number;
    get inert(): number;
    get friction(): number;
    get radius(): number;
    get endLength(): number;
    get endOffset(): Vector3;
    get gravity(): Vector3;
    get force(): Vector3;
    set root(transform: Transform);
    set updateRate(value: number);
    set updateMode(value: UpdateMode);
    set damping(value: number);
    set stiffness(value: number);
    set inert(value: number);
    set friction(value: number);
    set radius(value: number);
    set endLength(value: number);
    set elasticity(value: number);
    set gravity(value: Vector3);
    set force(value: Vector3);
    private static handle_check;
    private static getRoot;
    private static getUpdateRate;
    private static getUpdateMode;
    private static getDamping;
    private static getElasticity;
    private static getStiffness;
    private static getInert;
    private static getFriction;
    private static getRadius;
    private static getEndLength;
    private static getEndOffset;
    private static getGravity;
    private static getForce;
    private static setRoot;
    private static setUpdateRate;
    private static setUpdateMode;
    private static setDamping;
    private static setElasticity;
    private static setStiffness;
    private static setInert;
    private static setFriction;
    private static setRadius;
    private static setEndLength;
    private static setEndOffset;
    private static setGravity;
    private static setForce;
}
declare class DynamicBoneCollider extends Component {
    constructor(handle: never);
    get radius(): number;
    get height(): number;
    get radius2(): number;
    set radius(value: number);
    set height(value: number);
    set radius2(value: number);
    private static handle_check;
    private static getRadius;
    private static getHeight;
    private static getRadius2;
    private static setRadius;
    private static setHeight;
    private static setRadius2;
}
declare class BehaviorPrioritySelector extends BehaviorComposite {
    constructor(handle?: never);
}
declare class FABRIK extends IKSolver {
    constructor(handle: never);
    get target(): Transform | null;
    set target(transform: Transform);
    private static handle_check;
    private static getTarget;
    private static setTarget;
}
declare class NavMeshAgent extends Component {
    constructor(handle: never);
    get speed(): number;
    get angularSpeed(): number;
    get acceleration(): number;
    get stoppingDistance(): number;
    get autoBraking(): boolean;
    get baseOffset(): number;
    get radius(): number;
    get height(): number;
    get avoidancePriority(): number;
    get autoTraverseOffMeshLink(): boolean;
    get areaMask(): number;
    get velocity(): Vector3;
    get desiredVelocity(): Vector3;
    get destination(): Transform;
    get updatePosition(): boolean;
    get path(): NavMeshPath;
    get hasPath(): boolean;
    get isOnNavMesh(): boolean;
    get isOnOffMeshLink(): boolean;
    get isPathStale(): boolean;
    get isStopped(): boolean;
    get currentOffMeshLinkData(): OffMeshLinkData;
    get nextOffMeshLinkData(): OffMeshLinkData;
    get nextPosition(): Vector3;
    get remainingDistance(): number;
    get steeringTarget(): Vector3;
    get updateRotation(): boolean;
    get updateUpAxis(): boolean;
    get pathPending(): boolean;
    set speed(val: number);
    set angularSpeed(val: number);
    set acceleration(val: number);
    set stoppingDistance(val: number);
    set autoBraking(val: boolean);
    set baseOffset(val: number);
    set radius(val: number);
    set height(val: number);
    set avoidancePriority(val: number);
    set autoTraverseOffMeshLink(val: boolean);
    set areaMask(val: number);
    set destination(val: Transform);
    set updatePosition(val: boolean);
    set isStopped(val: boolean);
    set nextPosition(nextPos: Vector3);
    set updateRotation(val: boolean);
    set updateUpAxis(val: boolean);
    ActivateCurrentOffMeshLink(val: boolean): void;
    CalculatePath(targetPosition: Vector3, path: NavMeshPath): boolean;
    CompleteOffMeshLink(): void;
    FindClosestEdge(hitResult: NavMeshHit): boolean;
    GetAreaCost(idx: number): number;
    Move(offset: Vector3): void;
    Raycast(targetPosition: Vector3, hitResult: NavMeshHit): boolean;
    ResetPath(): void;
    SamplePathPosition(areaMask: number, maxDistance: number, hitRes: NavMeshHit): boolean;
    SetAreaCost(areaIndex: number, areaCost: number): void;
    private static handle_check;
    private static get_speed_impl;
    private static get_angular_velocity_impl;
    private static get_acceleration_impl;
    private static get_stop_dist_impl;
    private static get_auto_brake_impl;
    private static get_base_offset_impl;
    private static get_obstacle_radius_impl;
    private static get_obstacle_height_impl;
    private static get_priority_impl;
    private static get_activate_offmesh_impl;
    private static get_area_mask_impl;
    private static get_velocity_impl;
    private static get_desired_velocity_impl;
    private static get_target_impl;
    private static get_is_sync_pos_impl;
    private static get_cur_offmesh_link_data_impl;
    private static get_has_path_impl;
    private static get_is_on_navmesh_impl;
    private static get_is_on_offmesh_link_impl;
    private static get_is_path_stale_impl;
    private static get_is_stopped_impl;
    private static get_next_offmesh_link_data_impl;
    private static get_next_pos_impl;
    private static get_remaining_dist_impl;
    private static get_steering_target_impl;
    private static get_update_rot_impl;
    private static get_update_up_axis_impl;
    private static get_path_pending_impl;
    private static set_speed_impl;
    private static set_angular_velocity_impl;
    private static set_acc_speed_impl;
    private static set_stop_dist_impl;
    private static set_auto_brake_impl;
    private static set_base_offset_impl;
    private static set_obstacle_radius_impl;
    private static set_obstacle_height_impl;
    private static set_priority_impl;
    private static set_activate_offmesh_impl;
    private static set_area_mask_impl;
    private static set_target_impl;
    private static set_is_sync_pos_impl;
    private static set_is_stopped_impl;
    private static set_next_position_impl;
    private static set_update_rot_impl;
    private static set_update_up_axis_impl;
    private static get_path_impl;
    private static activated_cur_offmesh_link_impl;
    private static calculated_path_impl;
    private static complete_offmesh_impl;
    private static find_closest_edge_impl;
    private static get_area_cost_impl;
    private static move_impl;
    private static raycast_impl;
    private static reset_path_impl;
    private static sample_path_pos_impl;
    private static set_area_cost_impl;
}
declare class NavMeshObstacle extends Component {
    constructor(handle: never);
    get shape(): NavMeshObstacleShape;
    get center(): Vector3;
    get size(): Vector3;
    get radius(): number;
    get height(): number;
    get carvingTimeToStationary(): number;
    get carvingMoveThreshold(): number;
    get carveOnlyStationary(): boolean;
    get carving(): boolean;
    set shape(val: NavMeshObstacleShape);
    set center(val: Vector3);
    set size(val: Vector3);
    set radius(val: number);
    set height(val: number);
    set carvingTimeToStationary(val: number);
    set carvingMoveThreshold(val: number);
    set carveOnlyStationary(val: boolean);
    set carving(val: boolean);
    private static handle_check;
    private static get_shape_impl;
    private static get_center_offset_impl;
    private static get_extend_impl;
    private static get_radius_impl;
    private static get_height_impl;
    private static get_stationary_time_impl;
    private static get_move_threshold_impl;
    private static get_is_carve_on_stationary_impl;
    private static get_carve_impl;
    private static set_shape_impl;
    private static set_center_offset_impl;
    private static set_extend_impl;
    private static set_radius_impl;
    private static set_height_impl;
    private static set_stationary_time_impl;
    private static set_move_threshold_impl;
    private static set_is_carve_on_stationary_impl;
    private static set_carve_impl;
}
declare class ParticleSystem extends Component {
    constructor(handle: never);
    get playState(): PlayState;
    get startDelay(): PSCurve;
    get delayAllLoop(): boolean;
    get duration(): number;
    get simulationSpeed(): number;
    get randomSeed(): number;
    get autoRandomSeed(): boolean;
    get looping(): boolean;
    get playOnAwake(): boolean;
    get useScaledTime(): boolean;
    get simulationSpace(): PSSimulationSpace;
    get simulationSpaceCustom(): Transform;
    get emitterVelocityMode(): PSEmitterVelocityMode;
    get emitterVelocityCustom(): Vector3;
    get scalingMode(): PSScalingMode;
    get initialModuleLifetime(): PSCurve;
    get initialModuleSpeed(): PSCurve;
    get initialModuleColor(): PSGradient;
    get initialModuleSizeX(): PSCurve;
    get initialModuleSizeY(): PSCurve;
    get initialModuleSizeZ(): PSCurve;
    get initialModuleRotationX(): PSCurve;
    get initialModuleRotationY(): PSCurve;
    get initialModuleRotationZ(): PSCurve;
    get initialModuleFlipRotation(): number;
    get initialModuleMaxParticles(): number;
    get initialModuleUse3DSize(): boolean;
    get initialModuleUse3DRotation(): boolean;
    get emissionModuleEnable(): boolean;
    get emissionModuleBurstsCount(): number;
    get emissionModuleRateOverTime(): PSCurve;
    get emissionModuleRateOverDistance(): PSCurve;
    get shapeModuleEnable(): boolean;
    get shapeModuleAngle(): number;
    get shapeModuleLength(): number;
    get shapeModuleRadiusThickness(): number;
    get shapeModuleDonutRadius(): number;
    get shapeModuleBoxThickness(): Vector3;
    get shapeModulePosition(): Vector3;
    get shapeModuleRotation(): Vector3;
    get shapeModuleScale(): Vector3;
    get shapeModuleType(): PSShapeType;
    get shapeModuleStyle(): PSShapeStyle;
    get shapeModuleAlignToDirection(): boolean;
    get shapeModuleRandomizeDirection(): number;
    get shapeModuleSpherizeDirection(): number;
    get shapeModuleRandomizePosition(): number;
    get shapeModuleRadius(): any;
    get shapeModuleArc(): any;
    get velocityModuleEnable(): boolean;
    get velocityModuleVelocitySpace(): PSVelocitySpace;
    get velocityModuleLinearVelocityX(): PSCurve;
    get velocityModuleLinearVelocityY(): PSCurve;
    get velocityModuleLinearVelocityZ(): PSCurve;
    get velocityModuleOrbitalVelocityX(): PSCurve;
    get velocityModuleOrbitalVelocityY(): PSCurve;
    get velocityModuleOrbitalVelocityZ(): PSCurve;
    get velocityModuleRadialVelocity(): PSCurve;
    get velocityModuleOffsetX(): PSCurve;
    get velocityModuleOffsetY(): PSCurve;
    get velocityModuleOffsetZ(): PSCurve;
    get velocityModuleSpeedModifier(): PSCurve;
    get velocityLimitationModuleEnable(): boolean;
    get velocityLimitationModuleSeparateAxes(): boolean;
    get velocityLimitationModuleVelocitySpace(): PSVelocitySpace;
    get velocityLimitationModuleVelocity(): PSCurve;
    get velocityLimitationModuleVelocitySeparateX(): PSCurve;
    get velocityLimitationModuleVelocitySeparateY(): PSCurve;
    get velocityLimitationModuleVelocitySeparateZ(): PSCurve;
    get velocityLimitationModuleDrag(): PSCurve;
    get velocityLimitationModuleDragMultiplyBySize(): boolean;
    get velocityLimitationModuleDragMultiplyByVelocity(): boolean;
    get velocityLimitationModuleDampen(): number;
    get velocityInheritModuleEnable(): boolean;
    get velocityInheritModuleInheritMode(): PSVelocityInheritMode;
    get velocityInheritModuleMultiplier(): PSCurve;
    get forceModuleEnable(): boolean;
    get forceModuleSpace(): PSForceSpace;
    get forceModuleRandomize(): boolean;
    get forceModuleForceX(): PSCurve;
    get forceModuleForceY(): PSCurve;
    get forceModuleForceZ(): PSCurve;
    get colorModuleEnable(): boolean;
    get colorModuleColor(): PSGradient;
    get colorByVelocityModuleEnable(): boolean;
    get colorByVelocityModuleSpeedRange(): Vector2;
    get colorByVelocityModuleColor(): PSGradient;
    get sizeModuleEnable(): boolean;
    get sizeModuleSeparateAxes(): boolean;
    get sizeModuleSizeX(): PSCurve;
    get sizeModuleSizeY(): PSCurve;
    get sizeModuleSizeZ(): PSCurve;
    get sizeByVelocityModuleEnable(): boolean;
    get sizeByVelocityModuleSeparateAxes(): boolean;
    get sizeByVelocityModuleSpeedRange(): Vector2;
    get sizeByVelocityModuleSizeX(): PSCurve;
    get sizeByVelocityModuleSizeY(): PSCurve;
    get sizeByVelocityModuleSizeZ(): PSCurve;
    get rotationModuleEnable(): boolean;
    get rotationModuleSeparateAxes(): boolean;
    get rotationModuleAngularVelocityX(): PSCurve;
    get rotationModuleAngularVelocityY(): PSCurve;
    get rotationModuleAngularVelocityZ(): PSCurve;
    get rotationByVelocityModuleEnable(): boolean;
    get rotationByVelocityModuleSeparateAxes(): boolean;
    get rotationByVelocityModuleSpeedRange(): Vector2;
    get rotationByVelocityModuleAngularVelocityX(): PSCurve;
    get rotationByVelocityModuleAngularVelocityY(): PSCurve;
    get rotationByVelocityModuleAngularVelocityZ(): PSCurve;
    get renderModuleEnable(): boolean;
    get renderModuleRenderMode(): PSRenderMode;
    get renderModuleRenderSpace(): PSRenderSpace;
    get renderModuleSortMode(): PSSortMode;
    get renderModuleMaxParticleSize(): number;
    get renderModuleMinParticleSize(): number;
    get renderModuleCameraScale(): number;
    get renderModuleVelocityScale(): number;
    get renderModuleLengthScale(): number;
    get renderModuleNormalDirection(): number;
    get renderModulePivot(): Vector3;
    get renderModuleFlip(): Vector3;
    get renderModuleAllowRoll(): boolean;
    set startDelay(value: PSCurve);
    set delayAllLoop(value: boolean);
    set duration(value: number);
    set simulationSpeed(value: number);
    set randomSeed(value: number);
    set autoRandomSeed(value: boolean);
    set looping(value: boolean);
    set playOnAwake(value: boolean);
    set useScaledTime(value: boolean);
    set simulationSpace(value: number);
    set simulationSpaceCustom(value: Transform);
    set emitterVelocityMode(value: number);
    set emitterVelocityCustom(value: Vector3);
    set scalingMode(value: number);
    set initialModuleLifetime(value: PSCurve);
    set initialModuleSpeed(value: PSCurve);
    set initialModuleColor(value: PSGradient);
    set initialModuleSizeX(value: PSCurve);
    set initialModuleSizeY(value: PSCurve);
    set initialModuleSizeZ(value: PSCurve);
    set initialModuleRotationX(value: PSCurve);
    set initialModuleRotationY(value: PSCurve);
    set initialModuleRotationZ(value: PSCurve);
    set initialModuleFlipRotation(value: number);
    set initialModuleMaxParticles(value: number);
    set initialModuleUse3DSize(value: boolean);
    set initialModuleUse3DRotation(value: boolean);
    set emissionModuleEnable(value: boolean);
    set emissionModuleBurstsCount(value: number);
    set emissionModuleRateOverTime(value: PSCurve);
    set emissionModuleRateOverDistance(value: PSCurve);
    set shapeModuleEnable(value: boolean);
    set shapeModuleAngle(value: number);
    set shapeModuleLength(value: number);
    set shapeModuleRadiusThickness(value: number);
    set shapeModuleDonutRadius(value: number);
    set shapeModuleBoxThickness(value: Vector3);
    set shapeModulePosition(value: Vector3);
    set shapeModuleRotation(value: Vector3);
    set shapeModuleScale(value: Vector3);
    set shapeModuleType(value: number);
    set shapeModuleStyle(value: number);
    set shapeModuleRandomizeDirection(value: number);
    set shapeModuleSpherizeDirection(value: number);
    set shapeModuleRandomizePosition(value: number);
    set shapeModuleRadius(value: any);
    set shapeModuleArc(value: any);
    set shapeModuleAlignToDirection(value: boolean);
    set velocityModuleEnable(value: boolean);
    set velocityModuleVelocitySpace(value: number);
    set velocityModuleLinearVelocityX(value: PSCurve);
    set velocityModuleLinearVelocityY(value: PSCurve);
    set velocityModuleLinearVelocityZ(value: PSCurve);
    set velocityModuleOrbitalVelocityX(value: PSCurve);
    set velocityModuleOrbitalVelocityY(value: PSCurve);
    set velocityModuleOrbitalVelocityZ(value: PSCurve);
    set velocityModuleRadialVelocity(value: PSCurve);
    set velocityModuleOffsetX(value: PSCurve);
    set velocityModuleOffsetY(value: PSCurve);
    set velocityModuleOffsetZ(value: PSCurve);
    set velocityModuleSpeedModifier(value: PSCurve);
    set velocityLimitationModuleEnable(value: boolean);
    set velocityLimitationModuleSeparateAxes(value: boolean);
    set velocityLimitationModuleVelocitySpace(value: number);
    set velocityLimitationModuleVelocity(value: PSCurve);
    set velocityLimitationModuleVelocitySeparateX(value: PSCurve);
    set velocityLimitationModuleVelocitySeparateY(value: PSCurve);
    set velocityLimitationModuleVelocitySeparateZ(value: PSCurve);
    set velocityLimitationModuleDrag(value: PSCurve);
    set velocityLimitationModuleDragMultiplyBySize(value: boolean);
    set velocityLimitationModuleDragMultiplyByVelocity(value: boolean);
    set velocityLimitationModuleDampen(value: number);
    set velocityInheritModuleEnable(value: boolean);
    set velocityInheritModuleInheritMode(value: number);
    set velocityInheritModuleMultiplier(value: PSCurve);
    set forceModuleEnable(value: boolean);
    set forceModuleSpace(value: number);
    set forceModuleRandomize(value: boolean);
    set forceModuleForceX(value: PSCurve);
    set forceModuleForceY(value: PSCurve);
    set forceModuleForceZ(value: PSCurve);
    set colorModuleEnable(value: boolean);
    set colorModuleColor(value: PSGradient);
    set colorByVelocityModuleEnable(value: boolean);
    set colorByVelocityModuleSpeedRange(value: Vector2);
    set colorByVelocityModuleColor(value: PSGradient);
    set sizeModuleEnable(value: boolean);
    set sizeModuleSeparateAxes(value: boolean);
    set sizeModuleSizeX(value: PSCurve);
    set sizeModuleSizeY(value: PSCurve);
    set sizeModuleSizeZ(value: PSCurve);
    set sizeByVelocityModuleEnable(value: boolean);
    set sizeByVelocityModuleSeparateAxes(value: boolean);
    set sizeByVelocityModuleSpeedRange(value: Vector2);
    set sizeByVelocityModuleSizeX(value: PSCurve);
    set sizeByVelocityModuleSizeY(value: PSCurve);
    set sizeByVelocityModuleSizeZ(value: PSCurve);
    set rotationModuleEnable(value: boolean);
    set rotationModuleSeparateAxes(value: boolean);
    set rotationModuleAngularVelocityX(value: PSCurve);
    set rotationModuleAngularVelocityY(value: PSCurve);
    set rotationModuleAngularVelocityZ(value: PSCurve);
    set rotationByVelocityModuleEnable(value: boolean);
    set rotationByVelocityModuleSeparateAxes(value: boolean);
    set rotationByVelocityModuleSpeedRange(value: Vector2);
    set rotationByVelocityModuleAngularVelocityX(value: PSCurve);
    set rotationByVelocityModuleAngularVelocityY(value: PSCurve);
    set rotationByVelocityModuleAngularVelocityZ(value: PSCurve);
    set renderModuleEnable(value: boolean);
    set renderModuleRenderMode(value: number);
    set renderModuleRenderSpace(value: number);
    set renderModuleSortMode(value: number);
    set renderModuleMaxParticleSize(value: number);
    set renderModuleMinParticleSize(value: number);
    set renderModuleCameraScale(value: number);
    set renderModuleVelocityScale(value: number);
    set renderModuleLengthScale(value: number);
    set renderModuleNormalDirection(value: number);
    set renderModulePivot(value: Vector3);
    set renderModuleFlip(value: Vector3);
    set renderModuleAllowRoll(value: boolean);
    Play(): void;
    Pause(): void;
    Stop(value?: PSStopBehavior): void;
    private static handle_check;
    private static getPlayStateImpl;
    private static getStartDelayImpl;
    private static getDelayAllLoopImpl;
    private static getDurationImpl;
    private static getSimulationSpeedImpl;
    private static getRandomSeedImpl;
    private static getAutoRandomSeedImpl;
    private static getLoopingImpl;
    private static getPlayOnAwakeImpl;
    private static getUseScaledTimeImpl;
    private static getSimulationSpaceImpl;
    private static getSimulationSpaceCustomImpl;
    private static getEmitterVelocityModeImpl;
    private static getEmitterVelocityCustomImpl;
    private static getScalingModeImpl;
    private static getInitialModuleLifetimeImpl;
    private static getInitialModuleSpeedImpl;
    private static getInitialModuleColorImpl;
    private static getInitialModuleSizeXImpl;
    private static getInitialModuleSizeYImpl;
    private static getInitialModuleSizeZImpl;
    private static getInitialModuleRotationXImpl;
    private static getInitialModuleRotationYImpl;
    private static getInitialModuleRotationZImpl;
    private static getInitialModuleFlipRotationImpl;
    private static getInitialModuleMaxParticlesImpl;
    private static getInitialModuleUse3DSizeImpl;
    private static getInitialModuleUse3DRotationImpl;
    private static getEmissionModuleEnableImpl;
    private static getEmissionModuleBurstsCountImpl;
    private static getEmissionModuleRateOverTimeImpl;
    private static getEmissionModuleRateOverDistanceImpl;
    private static getShapeModuleEnableImpl;
    private static getShapeModuleAngleImpl;
    private static getShapeModuleLengthImpl;
    private static getShapeModuleRadiusThicknessImpl;
    private static getShapeModuleDonutRadiusImpl;
    private static getShapeModuleBoxThicknessImpl;
    private static getShapeModulePositionImpl;
    private static getShapeModuleRotationImpl;
    private static getShapeModuleScaleImpl;
    private static getShapeModuleTypeImpl;
    private static getShapeModuleStyleImpl;
    private static getShapeModuleAlignToDirectionImpl;
    private static getShapeModuleRandomizeDirectionImpl;
    private static getShapeModuleSpherizeDirectionImpl;
    private static getShapeModuleRandomizePositionImpl;
    private static getShapeModuleRadiusImpl;
    private static getShapeModuleArcImpl;
    private static getVelocityModuleEnableImpl;
    private static getVelocityModuleVelocitySpaceImpl;
    private static getVelocityModuleLinearVelocityXImpl;
    private static getVelocityModuleLinearVelocityYImpl;
    private static getVelocityModuleLinearVelocityZImpl;
    private static getVelocityModuleOrbitalVelocityXImpl;
    private static getVelocityModuleOrbitalVelocityYImpl;
    private static getVelocityModuleOrbitalVelocityZImpl;
    private static getVelocityModuleRadialVelocityImpl;
    private static getVelocityModuleOffsetXImpl;
    private static getVelocityModuleOffsetYImpl;
    private static getVelocityModuleOffsetZImpl;
    private static getVelocityModuleSpeedModifierImpl;
    private static getVelocityLimitationModuleEnableImpl;
    private static getVelocityLimitationModuleSeparateAxesImpl;
    private static getVelocityLimitationModuleVelocitySpaceImpl;
    private static getVelocityLimitationModuleVelocityImpl;
    private static getVelocityLimitationModuleVelocitySeparateXImpl;
    private static getVelocityLimitationModuleVelocitySeparateYImpl;
    private static getVelocityLimitationModuleVelocitySeparateZImpl;
    private static getVelocityLimitationModuleDragImpl;
    private static getVelocityLimitationModuleDragMultiplyBySizeImpl;
    private static getVelocityLimitationModuleDragMultiplyByVelocityImpl;
    private static getVelocityLimitationModuleDampenImpl;
    private static getVelocityInheritModuleEnableImpl;
    private static getVelocityInheritModuleInheritModeImpl;
    private static getVelocityInheritModuleMultiplierImpl;
    private static getForceModuleEnableImpl;
    private static getForceModuleSpaceImpl;
    private static getForceModuleRandomizeImpl;
    private static getForceModuleForceXImpl;
    private static getForceModuleForceYImpl;
    private static getForceModuleForceZImpl;
    private static getColorModuleEnableImpl;
    private static getColorModuleColorImpl;
    private static getColorByVelocityModuleEnableImpl;
    private static getColorByVelocityModuleSpeedRangeImpl;
    private static getColorByVelocityModuleColorImpl;
    private static getSizeModuleEnableImpl;
    private static getSizeModuleSeparateAxesImpl;
    private static getSizeModuleSizeXImpl;
    private static getSizeModuleSizeYImpl;
    private static getSizeModuleSizeZImpl;
    private static getSizeByVelocityModuleEnableImpl;
    private static getSizeByVelocityModuleSeparateAxesImpl;
    private static getSizeByVelocityModuleSpeedRangeImpl;
    private static getSizeByVelocityModuleSizeXImpl;
    private static getSizeByVelocityModuleSizeYImpl;
    private static getSizeByVelocityModuleSizeZImpl;
    private static getRotationModuleEnableImpl;
    private static getRotationModuleSeparateAxesImpl;
    private static getRotationModuleAngularVelocityXImpl;
    private static getRotationModuleAngularVelocityYImpl;
    private static getRotationModuleAngularVelocityZImpl;
    private static getRotationByVelocityModuleEnableImpl;
    private static getRotationByVelocityModuleSeparateAxesImpl;
    private static getRotationByVelocityModuleSpeedRangeImpl;
    private static getRotationByVelocityModuleAngularVelocityXImpl;
    private static getRotationByVelocityModuleAngularVelocityYImpl;
    private static getRotationByVelocityModuleAngularVelocityZImpl;
    private static getRenderModuleEnableImpl;
    private static getRenderModuleRenderModeImpl;
    private static getRenderModuleRenderSpaceImpl;
    private static getRenderModuleSortModeImpl;
    private static getRenderModuleMaxParticleSizeImpl;
    private static getRenderModuleMinParticleSizeImpl;
    private static getRenderModuleCameraScaleImpl;
    private static getRenderModuleVelocityScaleImpl;
    private static getRenderModuleLengthScaleImpl;
    private static getRenderModuleNormalDirectionImpl;
    private static getRenderModulePivotImpl;
    private static getRenderModuleFlipImpl;
    private static getRenderModuleAllowRollImpl;
    private static setStartDelayImpl;
    private static setDelayAllLoopImpl;
    private static setDurationImpl;
    private static setSimulationSpeedImpl;
    private static setRandomSeedImpl;
    private static setAutoRandomSeedImpl;
    private static setLoopingImpl;
    private static setPlayOnAwakeImpl;
    private static setUseScaledTimeImpl;
    private static setSimulationSpaceImpl;
    private static setSimulationSpaceCustomImpl;
    private static setEmitterVelocityModeImpl;
    private static setEmitterVelocityCustomImpl;
    private static setScalingModeImpl;
    private static setInitialModuleLifetimeImpl;
    private static setInitialModuleSpeedImpl;
    private static setInitialModuleColorImpl;
    private static setInitialModuleSizeXImpl;
    private static setInitialModuleSizeYImpl;
    private static setInitialModuleSizeZImpl;
    private static setInitialModuleRotationXImpl;
    private static setInitialModuleRotationYImpl;
    private static setInitialModuleRotationZImpl;
    private static setInitialModuleFlipRotationImpl;
    private static setInitialModuleMaxParticlesImpl;
    private static setInitialModuleUse3DSizeImpl;
    private static setInitialModuleUse3DRotationImpl;
    private static setEmissionModuleEnableImpl;
    private static setEmissionModuleBurstsCountImpl;
    private static setEmissionModuleRateOverTimeImpl;
    private static setEmissionModuleRateOverDistanceImpl;
    private static setShapeModuleEnableImpl;
    private static setShapeModuleAngleImpl;
    private static setShapeModuleLengthImpl;
    private static setShapeModuleRadiusThicknessImpl;
    private static setShapeModuleDonutRadiusImpl;
    private static setShapeModuleBoxThicknessImpl;
    private static setShapeModulePositionImpl;
    private static setShapeModuleRotationImpl;
    private static setShapeModuleScaleImpl;
    private static setShapeModuleTypeImpl;
    private static setShapeModuleStyleImpl;
    private static setShapeModuleRandomizeDirectionImpl;
    private static setShapeModuleSpherizeDirectionImpl;
    private static setShapeModuleRandomizePositionImpl;
    private static setShapeModuleRadiusImpl;
    private static setShapeModuleArcImpl;
    private static setShapeModuleAlignToDirectionImpl;
    private static setVelocityModuleEnableImpl;
    private static setVelocityModuleVelocitySpaceImpl;
    private static setVelocityModuleLinearVelocityXImpl;
    private static setVelocityModuleLinearVelocityYImpl;
    private static setVelocityModuleLinearVelocityZImpl;
    private static setVelocityModuleOrbitalVelocityXImpl;
    private static setVelocityModuleOrbitalVelocityYImpl;
    private static setVelocityModuleOrbitalVelocityZImpl;
    private static setVelocityModuleRadialVelocityImpl;
    private static setVelocityModuleOffsetXImpl;
    private static setVelocityModuleOffsetYImpl;
    private static setVelocityModuleOffsetZImpl;
    private static setVelocityModuleSpeedModifierImpl;
    private static setVelocityLimitationModuleEnableImpl;
    private static setVelocityLimitationModuleSeparateAxesImpl;
    private static setVelocityLimitationModuleVelocitySpaceImpl;
    private static setVelocityLimitationModuleVelocityImpl;
    private static setVelocityLimitationModuleVelocitySeparateXImpl;
    private static setVelocityLimitationModuleVelocitySeparateYImpl;
    private static setVelocityLimitationModuleVelocitySeparateZImpl;
    private static setVelocityLimitationModuleDragImpl;
    private static setVelocityLimitationModuleDragMultiplyBySizeImpl;
    private static setVelocityLimitationModuleDragMultiplyByVelocityImpl;
    private static setVelocityLimitationModuleDampenImpl;
    private static setVelocityInheritModuleEnableImpl;
    private static setVelocityInheritModuleInheritModeImpl;
    private static setVelocityInheritModuleMultiplierImpl;
    private static setForceModuleEnableImpl;
    private static setForceModuleSpaceImpl;
    private static setForceModuleRandomizeImpl;
    private static setForceModuleForceXImpl;
    private static setForceModuleForceYImpl;
    private static setForceModuleForceZImpl;
    private static setColorModuleEnableImpl;
    private static setColorModuleColorImpl;
    private static setColorByVelocityModuleEnableImpl;
    private static setColorByVelocityModuleSpeedRangeImpl;
    private static setColorByVelocityModuleColorImpl;
    private static setSizeModuleEnableImpl;
    private static setSizeModuleSeparateAxesImpl;
    private static setSizeModuleSizeXImpl;
    private static setSizeModuleSizeYImpl;
    private static setSizeModuleSizeZImpl;
    private static setSizeByVelocityModuleEnableImpl;
    private static setSizeByVelocityModuleSeparateAxesImpl;
    private static setSizeByVelocityModuleSpeedRangeImpl;
    private static setSizeByVelocityModuleSizeXImpl;
    private static setSizeByVelocityModuleSizeYImpl;
    private static setSizeByVelocityModuleSizeZImpl;
    private static setRotationModuleEnableImpl;
    private static setRotationModuleSeparateAxesImpl;
    private static setRotationModuleAngularVelocityXImpl;
    private static setRotationModuleAngularVelocityYImpl;
    private static setRotationModuleAngularVelocityZImpl;
    private static setRotationByVelocityModuleEnableImpl;
    private static setRotationByVelocityModuleSeparateAxesImpl;
    private static setRotationByVelocityModuleSpeedRangeImpl;
    private static setRotationByVelocityModuleAngularVelocityXImpl;
    private static setRotationByVelocityModuleAngularVelocityYImpl;
    private static setRotationByVelocityModuleAngularVelocityZImpl;
    private static setRenderModuleEnableImpl;
    private static setRenderModuleRenderModeImpl;
    private static setRenderModuleRenderSpaceImpl;
    private static setRenderModuleSortModeImpl;
    private static setRenderModuleMaxParticleSizeImpl;
    private static setRenderModuleMinParticleSizeImpl;
    private static setRenderModuleCameraScaleImpl;
    private static setRenderModuleVelocityScaleImpl;
    private static setRenderModuleLengthScaleImpl;
    private static setRenderModuleNormalDirectionImpl;
    private static setRenderModulePivotImpl;
    private static setRenderModuleFlipImpl;
    private static setRenderModuleAllowRollImpl;
    private static PlayImpl;
    private static PauseImpl;
    private static StopImpl;
}
declare class PostEffectPackage extends Component {
    constructor(handle: never);
    get postEffects(): PostEffectBase[];
    GetPostEffectByDefine(value: PostEffectDefine): PostEffectBase;
    AddPostEffect(value: PostEffectDefine): PostEffectBase;
    AddPostEffectObject(value: PostEffectBase): void;
    SetEnable(value: PostEffectDefine, enable: boolean): void;
    GetEnable(value: PostEffectDefine): boolean;
    private static handle_check;
    private static AddPostEffect_impl;
    private static AddPostEffectObject_impl;
    private static setEnable_impl;
    private static getEnable_impl;
    private static getPostEffects_impl;
    private static getPostEffectByDefine_impl;
}
declare class Ragdoll extends Component {
    constructor(handle: never);
    AddForce(force: Vector3, mode: ForceMode): void;
    private static handle_check;
    private static AddForce_impl;
}
declare class SkinnedMeshRenderer extends Renderer {
    constructor(handle: never);
    get sharedMesh(): Mesh | null;
    get bones(): Transform[] | null;
    get rootBone(): Transform | null;
    set sharedMesh(value: Mesh);
    set bones(bones: Transform[]);
    set rootBone(bone: Transform);
    private static handle_check;
    private static getSharedMesh_impl;
    private static getBones_impl;
    private static getRootBone_impl;
    private static setSharedMesh_impl;
    private static setBones_impl;
    private static setRootBone_impl;
}
declare class BehaviorSendEvent extends BehaviorAction {
    constructor(handle?: never);
    get eventName(): BehaviorString;
    get argument0(): BehaviorVariable;
    get argument1(): BehaviorVariable;
    get argument2(): BehaviorVariable;
    set eventName(val: BehaviorString);
    set argument0(param: BehaviorVariable);
    set argument1(param: BehaviorVariable);
    set argument2(param: BehaviorVariable);
    SetEventName(name: string): void;
    private static get_event_name;
    private static set_event_name;
    private static set_event_name_var;
    private static get_argument_impl;
    private static set_argument_impl;
}
declare class SphereCollider extends Collider {
    constructor(handle: never);
    get center(): Vector3;
    get radius(): number;
    set center(value: Vector3);
    set radius(radius: number);
    private static handle_check;
    private static getCenter;
    private static getRadius;
    private static setCenter;
    private static setRadius;
}
declare class SpriteSequenceRenderer extends SpriteRenderer {
    constructor(handle: never);
    get spriteSequence(): SpriteSequence | null;
    get loopCount(): number;
    get loop(): boolean;
    get isPlaying(): boolean;
    get isStop(): boolean;
    get isPause(): boolean;
    set spriteSequence(value: SpriteSequence | null);
    set loopCount(value: number);
    set loop(value: boolean);
    Reset(): void;
    Play(): void;
    Stop(): void;
    Continue(): void;
    Pause(): void;
    GoToSprite(value: number): void;
    PreviousSprite(): void;
    NextSprite(): void;
    private static handle_check_impl;
    private static get_sprite_sequence;
    private static get_loop_count;
    private static is_loop;
    private static is_playing;
    private static is_stop;
    private static is_pause;
    private static set_sprite_sequence;
    private static set_loop_count;
    private static set_loop;
    private static reset_impl;
    private static play_impl;
    private static stop_impl;
    private static continue_impl;
    private static pause_impl;
    private static go_to_sprite_impl;
    private static previous_sprite_impl;
    private static next_sprite_impl;
}
declare class UIComponent extends Component {
    constructor(handle: never);
    get guiPrefab(): GUIPrefab;
    get uiMode(): CanvasMode;
    get uiOrder(): number;
    get uiSortingLayer(): number;
    get uiFaceCamera(): boolean;
    get uiOccludedBy3D(): boolean;
    get canvas(): Canvas;
    get uiPixelPerUnitX(): number;
    get uiPixelPerUnitY(): number;
    get eventCamera(): Camera;
    set guiPrefab(guiPrefab: GUIPrefab);
    set uiMode(canvasMode: CanvasMode);
    set uiOrder(order: number);
    set uiSortingLayer(layer: number);
    set uiOccludedBy3D(faceCamera: boolean);
    set uiFaceCamera(faceCamera: boolean);
    set uiPixelPerUnitX(ppu: number);
    set uiPixelPerUnitY(ppu: number);
    set eventCamera(camrea: Camera);
    private static handle_check;
    private static get_gui_prefab;
    private static get_ui_mode;
    private static get_ui_order;
    private static get_ui_sorting_layer;
    private static get_ui_face_camera;
    private static get_ui_occluded_by_3d;
    private static get_pixel_per_unit_x;
    private static get_pixel_per_unit_y;
    private static get_canvas;
    private static get_event_camera;
    private static set_gui_prefab;
    private static set_ui_mode;
    private static set_ui_order;
    private static set_ui_sorting_layer;
    private static set_ui_occluded_by_3d;
    private static set_ui_face_camera;
    private static set_pixel_per_unit_x;
    private static set_pixel_per_unit_y;
    private static set_event_camera;
}
declare class VehicleCreator extends Rigidbody {
}
declare class WheelCollider extends Component {
    constructor(handle: never);
    get modelTransform(): Transform | null;
    get isGrounded(): number;
    get Rpm(): number;
    set modelTransform(transform: Transform);
    set MotorTorque(value: number);
    set BrakeTorque(value: number);
    set SteerAngle(value: number);
    private static handle_check;
    private static getmodelTransform_impl;
    private static getIsGrounded_impl;
    private static getRpm_impl;
    private static setmodelTransform_impl;
    private static setMotorTorque_impl;
    private static setBrakeTorque_impl;
    private static setSteerAngle_impl;
}
declare class NavMesh implements NavMeshHitUser {
    static SamplePosition(srcPos: Vector3, maxDist: number, areaMask: number): NavMeshHit;
    static CalculatePath(sourcePosition: Vector3, targetPosition: Vector3, areaMask: number, path: NavMeshPath): boolean;
    static FindClosestEdge(sourcePosition: Vector3, hitResult: NavMeshHit, areaMask: number): boolean;
    static GetAreaCost(areaIndex: number): number;
    static GetAreaFromName(areaName: string): number;
    static Raycast(sourcePosition: Vector3, targetPosition: Vector3, hitResult: NavMeshHit, areaMask: number): boolean;
    static SetAreaCost(areaIndex: number, areaCost: number): void;
    private static sample_pos_impl;
    private static calculate_path_impl;
    private static find_closest_edge_impl;
    private static get_area_cost;
    private static get_area_from_name;
    private static raycast_impl;
    private static set_area_cost_impl;
}
declare class Toggle extends TransitionControl {
    constructor(handle: never);
    get bg(): Control;
    get label(): Control;
    get checkmark(): Control;
    get isOn(): boolean;
    get group(): string;
    set isOn(enable: boolean);
    set group(group: string);
    AddEvent(event: ToggleEvent | ControlBaseEvent | ControlEvent, callBack: IControlEventCallBack | IControlEventCallBackI | IControlEventCallBackS): void;
    ClearEvent(event: ToggleEvent | ControlBaseEvent | ControlEvent, callBack?: IControlEventCallBack | IControlEventCallBackI | IControlEventCallBackS | null): void;
    private static handle_check;
    private static get_bg_control;
    private static get_label_control;
    private static get_checkmark_control;
    private static get_is_on;
    private static get_group;
    private static set_is_on;
    private static set_group;
}
declare class NavMeshPath {
    private _corners;
    private _status;
    get corners(): Vector3[];
    get status(): NavMeshPathStatus;
    set corners(arr: Vector3[]);
    set status(val: NavMeshPathStatus);
}
declare class PSCurve {
    private _handle;
    constructor(handle?: never);
    get curveMode(): PSCurveMode;
    get maxCurve(): FloatCurve | null;
    get maxScalar(): number;
    get minCurve(): FloatCurve | null;
    get minScalar(): number;
    get isOptimized(): boolean;
    set curveMode(value: PSCurveMode);
    set maxCurveWrapMode(value: WrapMode);
    set minCurveWrapMode(value: WrapMode);
    set maxScalar(value: number);
    set minScalar(value: number);
    Evaluate(time: number, random_value?: number): number;
    BuildCurves(): boolean;
    CopyTo(target: PSCurve): void;
    Reset(mode?: PSCurveMode, scalar?: number, curve_begin_value?: number, curve_end_value?: number): void;
    private static alloc_object;
    private static handle_check;
    private static get_curve_mode;
    private static get_max_curve;
    private static get_max_scalar;
    private static get_min_curve;
    private static get_min_scalar;
    private static get_is_optimized;
    private static set_curve_mode;
    private static set_max_curve_wrap_mode;
    private static set_min_curve_wrap_mode;
    private static set_max_scalar;
    private static set_min_scalar;
    private static evaluate;
    private static build_curves;
    private static copy_to;
    private static reset;
}
declare class BehaviorTreeReference extends BehaviorTask {
    constructor(handle?: never);
    get ExternalBehavior(): ExternalBehaviorTree;
    set ExternalBehavior(externalBehavior: ExternalBehaviorTree);
    private static get_ext_behavior;
    private static set_ext_behavior;
}
declare class Bloom extends PostEffectBase {
    constructor(handle?: never);
    get intensity(): number;
    get threshold(): number;
    get anamorphicRatio(): number;
    get color(): Color;
    get max_iteration_count(): number;
    get scatter(): number;
    set intensity(val: number);
    set threshold(val: number);
    set anamorphicRatio(val: number);
    set color(val: Color);
    set max_iteration_count(val: number);
    set scatter(val: number);
    private static alloc_object_impl;
    private static handle_check;
    private static get_intensity_impl;
    private static get_threshold_impl;
    private static get_anamorphicRatio_impl;
    private static get_color_impl;
    private static get_max_iteration_count_impl;
    private static get_scatter_impl;
    private static set_intensity_impl;
    private static set_threshold_impl;
    private static set_anamorphicRatio_impl;
    private static set_color_impl;
    private static set_max_iteration_count_impl;
    private static set_scatter_impl;
}
declare class NetDBSetter {
    private _handle;
    constructor(handle: never);
    get isDone(): boolean;
    SetValue(user_id: number, key: string, value: string): boolean;
    Clear(): boolean;
    Commit(): Promise<void>;
    private static handle_check;
    private static get_is_query_done;
    private static set_value_impl;
    private static clear_impl;
    private static commit_impl;
}
declare class SequenceOperator extends TweenOperator {
    constructor(handle: never);
    TimeScale(value: number): SequenceOperator;
    LoopCount(value: number): SequenceOperator;
    AddStartCallBack(value: ITweenCallBack): SequenceOperator;
    AddUpdateCallBack(value: ITweenCallBack): SequenceOperator;
    AddStepCompletedCallBack(value: ITweenCallBack): SequenceOperator;
    AddFinishCallBack(value: ITweenCallBack): SequenceOperator;
    Invalid(): boolean;
    AddDelay(value: number): SequenceOperator;
    Add(value: TweenerOperator): SequenceOperator;
    AddList(valueList: TweenerOperator[]): SequenceOperator;
    AddParallel(valueList: TweenerOperator[]): SequenceOperator;
    AddCallBack(value: ITweenerCallBack): SequenceOperator;
    private invalid;
    private static handle_check;
    private static add_delay;
    private static add;
    private static start_parallel;
    private static add_parallel;
    private static end_parallel;
    private static add_call_back;
}
declare class TweenerOperator extends TweenOperator {
    constructor(handle: never);
    TimeScale(value: number): TweenerOperator;
    LoopCount(value: number): TweenerOperator;
    AddStartCallBack(value: ITweenCallBack): TweenerOperator;
    AddUpdateCallBack(value: ITweenCallBack): TweenerOperator;
    AddStepCompletedCallBack(value: ITweenCallBack): TweenerOperator;
    AddFinishCallBack(value: ITweenCallBack): TweenerOperator;
    Invalid(): boolean;
    Delay(value: number): TweenerOperator;
    AutoKill(value: boolean): TweenerOperator;
    Duration(value: number): TweenerOperator;
    EaseType(value: TweenEaseType): TweenerOperator;
    LoopType(value: TweenLoopType): TweenerOperator;
    EaseFunc(value: IEaseFunction): TweenerOperator;
    From(value: number | Vector2 | Vector3 | Color): TweenerOperator;
    private invalid;
    private static handle_check;
    private static set_delay_time;
    private static set_auto_kill;
    private static set_duration;
    private static set_ease_type;
    private static set_loop_type;
    private static set_ease_func;
    private static set_number_from;
    private static set_vector2_from;
    private static set_vector3_from;
    private static set_color_from;
}
declare class BehaviorLogFormat extends BehaviorAction {
    constructor(handle?: never);
    get argument0(): BehaviorVariable;
    get argument1(): BehaviorVariable;
    get argument2(): BehaviorVariable;
    get argument3(): BehaviorVariable;
    set argument0(param: BehaviorVariable);
    set argument1(param: BehaviorVariable);
    set argument2(param: BehaviorVariable);
    set argument3(param: BehaviorVariable);
    private static get_argument_impl;
    private static set_argument_impl;
}
declare class BehaviorPerformInterruption extends BehaviorAction {
    constructor(handle?: never);
    get interruptTasks(): BehaviorInterrupt[];
    get interruptSuccess(): BehaviorBool;
    set interruptSuccess(val: BehaviorBool);
    SetInterruptTasks(list: BehaviorInterrupt[]): void;
    SetInterruptSuccess(val: boolean): void;
    private static get_interrupt_tasks;
    private static get_interrupt_suc;
    private static set_interrupt_tasks;
    private static set_interrupt_suc;
    private static set_interrupt_suc_var;
}
declare class BehaviorWait extends BehaviorAction {
    constructor(handle?: never);
    get time(): BehaviorFloat;
    get randomWait(): BehaviorBool;
    get randomWaitMin(): BehaviorFloat;
    get randomWaitMax(): BehaviorFloat;
    set time(val: BehaviorFloat);
    set randomWait(val: BehaviorBool);
    set randomWaitMin(val: BehaviorFloat);
    set randomWaitMax(val: BehaviorFloat);
    SetTime(time: number): void;
    SetRandomWait(val: boolean): void;
    SetRandomWaitMin(min: number): void;
    SetRandomWaitMax(max: number): void;
    private static get_time;
    private static get_random_wait;
    private static get_random_wait_min;
    private static get_random_wait_max;
    private static set_time;
    private static set_random_wait;
    private static set_random_wait_min;
    private static set_random_wait_max;
    private static set_time_var;
    private static set_random_wait_var;
    private static set_random_wait_min_var;
    private static set_random_wait_max_var;
}
declare class RestartBehaviorTree extends BehaviorAction {
    constructor(handle?: never);
    get behaviorGameObject(): BehaviorGameObject;
    set behaviorGameObject(val: BehaviorGameObject | GameObject);
    SetBehaviorGameObject(obj: GameObject): void;
    private static get_behavior;
    private static set_behavior;
    private static set_behavior_var;
}
declare class BehaviorParallel extends BehaviorComposite {
    constructor(handle?: never);
}
declare class BehaviorParallelSelector extends BehaviorComposite {
    constructor(handle?: never);
}
declare class BehaviorRandomSelector extends BehaviorComposite {
    constructor(handle?: never);
    get seed(): BehaviorInt;
    get useSeed(): BehaviorBool;
    set seed(val: BehaviorInt);
    set useSeed(val: BehaviorBool);
    SetSeed(seed: number): void;
    SetUseSeed(use_seed: boolean): void;
    private static get_seed_var;
    private static get_use_seed_var;
    private static set_seed_var;
    private static set_use_seed_var;
    private static set_seed_value;
    private static set_use_seed_value;
}
declare class BehaviorRandomSequence extends BehaviorComposite {
    constructor(handle?: never);
    get seed(): BehaviorInt;
    get useSeed(): BehaviorBool;
    set seed(val: BehaviorInt);
    set useSeed(val: BehaviorBool);
    SetSeed(seed: number): void;
    SetUseSeed(use_seed: boolean): void;
    private static get_seed_var;
    private static get_use_seed_var;
    private static set_seed_var;
    private static set_use_seed_var;
    private static set_seed_value;
    private static set_use_seed_value;
}
declare class BehaviorSelector extends BehaviorComposite {
    constructor(handle?: never);
}
declare class BehaviorSelectorEvaluator extends BehaviorComposite {
    constructor(handle?: never);
}
declare class FBBIK extends IKSolver {
    constructor(handle: never);
    get spineTarget(): Transform | null;
    get leftHandTarget(): Transform | null;
    get leftShoulderTarget(): Transform | null;
    get rightHandTarget(): Transform | null;
    get rightShoulderTarget(): Transform | null;
    get leftFootTarget(): Transform | null;
    get leftThighTarget(): Transform | null;
    get rightFootTarget(): Transform | null;
    get rightThighTarget(): Transform | null;
    get spineWeight(): number;
    get leftHandWeight(): number;
    get leftShoulderWeight(): number;
    get rightHandWeight(): number;
    get rightShoulderWeight(): number;
    get leftFootWeight(): number;
    get leftThighWeight(): number;
    get rightFootWeight(): number;
    get rightThighWeight(): number;
    get spineStiffness(): number;
    set spineTarget(transform: Transform);
    set leftHandTarget(transform: Transform);
    set leftShoulderTarget(transform: Transform);
    set rightHandTarget(transform: Transform);
    set rightShoulderTarget(transform: Transform);
    set leftFootTarget(transform: Transform);
    set leftThighTarget(transform: Transform);
    set rightFootTarget(transform: Transform);
    set rightThighTarget(transform: Transform);
    set spineWeight(weight: number);
    set leftHandWeight(weight: number);
    set leftShoulderWeight(weight: number);
    set rightHandWeight(weight: number);
    set rightShoulderWeight(weight: number);
    set leftFootWeight(weight: number);
    set leftThighWeight(weight: number);
    set rightFootWeight(weight: number);
    set rightThighWeight(weight: number);
    set spineStiffness(weight: number);
    private static handle_check;
    private static getSpineTarget;
    private static getLeftHandTarget;
    private static getLeftShoulderTarget;
    private static getRightHandTarget;
    private static getRightShoulderTarget;
    private static getLeftFootTarget;
    private static getLeftThighTarget;
    private static getRightFootTarget;
    private static getRightThighTarget;
    private static getSpineWeight;
    private static getLeftHandWeight;
    private static getLeftShoulderWeight;
    private static getRightHandWeight;
    private static getRightShoulderWeight;
    private static getLeftFootWeight;
    private static getLeftThighWeight;
    private static getRightFootWeight;
    private static getRightThighWeight;
    private static getSpineStiffness;
    private static setSpineTarget;
    private static setLeftHandTarget;
    private static setLeftShoulderTarget;
    private static setRightHandTarget;
    private static setRightShoulderTarget;
    private static setLeftFootTarget;
    private static setLeftThighTarget;
    private static setRightFootTarget;
    private static setRightThighTarget;
    private static setSpineWeight;
    private static setLeftHandWeight;
    private static setLeftShoulderWeight;
    private static setRightHandWeight;
    private static setRightShoulderWeight;
    private static setLeftFootWeight;
    private static setLeftThighWeight;
    private static setRightFootWeight;
    private static setRightThighWeight;
    private static setSpineStiffness;
}
declare class BehaviorHasReceivedEvent extends BehaviorConditional {
    constructor(handle?: never);
    get eventName(): BehaviorString;
    get argument0(): BehaviorVariable;
    get argument1(): BehaviorVariable;
    get argument2(): BehaviorVariable;
    set eventName(val: BehaviorString);
    set argument0(param: BehaviorVariable);
    set argument1(param: BehaviorVariable);
    set argument2(param: BehaviorVariable);
    SetEventName(name: string): void;
    private static get_event_name;
    private static set_event_name;
    private static set_event_name_var;
    private static get_argument_impl;
    private static set_argument_impl;
}
declare class BehaviorRandomProbability extends BehaviorConditional {
    constructor(handle?: never);
    get successProbability(): BehaviorFloat;
    get seed(): BehaviorInt;
    get useSeed(): BehaviorBool;
    set successProbability(val: BehaviorFloat);
    set seed(val: BehaviorInt);
    set useSeed(val: BehaviorBool);
    SetSuccessProbability(probability: number): void;
    SetSeed(seed: number): void;
    SetUseSeed(use_seed: boolean): void;
    private static get_suc_prob;
    private static set_suc_prob;
    private static set_suc_prob_var;
    private static get_seed_var;
    private static get_use_seed_var;
    private static set_seed_var;
    private static set_use_seed_var;
    private static set_seed_value;
    private static set_use_seed_value;
}
declare class BehaviorConditionalEvaluator extends BehaviorDecorator {
    constructor(handle?: never);
    get reevaluate(): BehaviorBool;
    get conditionalTask(): BehaviorConditional;
    set reevaluate(val: BehaviorBool);
    set conditionalTask(task: BehaviorConditional);
    SetReevaluate(val: boolean): void;
    private static get_reevaluate;
    private static get_condition;
    private static set_reevaluate;
    private static set_reevaluate_var;
    private static set_condition;
}
declare class BehaviorInterrupt extends BehaviorDecorator {
    constructor(handle?: never);
}
declare class BehaviorInverter extends BehaviorDecorator {
    constructor(handle?: never);
}
declare class BehaviorReturnSuccess extends BehaviorDecorator {
    constructor(handle?: never);
}
declare enum TouchPhase {
    None = 0,
    Began = 1,
    Moved = 2,
    Stationary = 3,
    Ended = 4,
    Canceled = 5
}
declare enum BonesPerVertex {
    BonesPerVertexNone = 0,
    BonesPerVertex1 = 1,
    BonesPerVertex2 = 2,
    BonesPerVertex4 = 4,
    BonesPerVertexVariable = 255
}
declare enum ShaderLevel {
    Low = 0,
    Medium = 1,
    High = 2
}
declare enum NetEntitySyncMask {
    None = 0,
    PosX = 1,
    PosY = 2,
    PosZ = 4,
    Position = 7,
    SclX = 64,
    SclY = 128,
    SclZ = 256,
    Scale = 448,
    Animate = 512,
    Enable = 1024,
    Layer = 2048,
    RigidBody = 4096,
    RigidBodyAngle = 8192,
    RigidBodyVelc = 16384,
    Assignable = 32768,
    QuatX = 65536,
    QuatY = 131072,
    QuatZ = 262144,
    QuatW = 524288,
    Rotation = 983040,
    GoParent = 1048576,
    Name = 2097152,
    Userdata = 4194304,
    Default = 8388551
}
declare enum ControlEvent {
    EventClick = 0,
    EventPointerEnter = 1,
    EventPointerLeave = 2,
    EventPointerUp = 3,
    EventPointerDown = 4,
    EventScrollWheel = 5,
    EventPointerDrag = 6,
    EventPointerStationary = 7,
    EventSelected = 8,
    EventLostSelected = 9,
    EventShow = 10,
    EventHidden = 11,
    EventEnabled = 12,
    EventDisabled = 13,
    EventKeyDown = 14,
    EventKeyPress = 15,
    EventKeyUp = 16,
    EventCharacterKey = 17,
    EventStart = 18,
    EventUpdated = 19,
    EventDestruction = 20,
    EventAddChild = 21,
    EventRemoveChild = 22,
    EventLayoutResize = 23,
    EventRichTextClick = 24,
    EventSelectChanged = 25,
    EventValueChanged = 26,
    EventScroll = 27,
    EventContentMove = 28,
    EventPullDown = 29,
    EventPullUp = 30,
    EventPullLeft = 31,
    EventPullRight = 32,
    EventValidityChanged = 33,
    EventValidityChangedState = 34,
    EventStateChanged = 35,
    MAX = 36
}
declare enum GUIMaskEvent {
    Click = 0,
    ScrollWheel = 5,
    PointerDrag = 6,
    PointerStationary = 7,
    KeyDown = 14,
    KeyPress = 15,
    KeyUp = 16,
    PointerEnter = 1,
    PointerLeave = 2,
    PointerUp = 3,
    PointerDown = 4,
    Selected = 8,
    LostSelected = 9,
    Show = 10,
    Hidden = 11,
    Enabled = 12,
    Disabled = 13,
    Start = 18,
    Updated = 19,
    Destruction = 20,
    AddChild = 21,
    RemoveChild = 22,
    LayoutResize = 23
}
declare enum CloneMode {
    Auto = 0,
    ForceDeepCopy = 1,
    ForceShallowCopy = 2
}
declare enum FileAccess {
    Read = 1,
    Write = 2,
    ReadWrite = 3
}
declare enum FileDirection {
    Start = 1,
    End = 2
}
declare enum FileMode {
    Create = 1,
    Open = 2,
    OpenOrCreate = 3,
    Truncate = 4
}
declare enum IndexFormat {
    Format16 = 0,
    Format32 = 1
}
declare enum TextEvent {
    RichTextClick = 24,
    Click = 0,
    ScrollWheel = 5,
    PointerDrag = 6,
    PointerStationary = 7,
    KeyDown = 14,
    KeyPress = 15,
    KeyUp = 16,
    PointerEnter = 1,
    PointerLeave = 2,
    PointerUp = 3,
    PointerDown = 4,
    Selected = 8,
    LostSelected = 9,
    Show = 10,
    Hidden = 11,
    Enabled = 12,
    Disabled = 13,
    Start = 18,
    Updated = 19,
    Destruction = 20,
    AddChild = 21,
    RemoveChild = 22,
    LayoutResize = 23
}
declare enum AudioRolloffMode {
    Logarithmic = 0,
    Linear = 1,
    Custom = 2,
    Count = 3
}
declare enum GradientBlendMode {
    Blend = 0,
    Fixed = 1
}
declare enum VisibilityType {
    AutoHideAndExpand = 0,
    AutoHide = 1,
    Permanent = 2
}
declare enum TextBoxTrigger {
    None = -1,
    Send = 0,
    Inputting = 1,
    Quit = 2
}
declare enum ButtonState {
    None = 0,
    Down = 1,
    Up = 2,
    Pressed = 3
}
declare enum CameraClearFlags {
    Skybox = 1,
    Color = 2,
    Depth = 3,
    Nothing = 4
}
declare enum LayoutFillLimitType {
    Auto = 0,
    Row = 1,
    Column = 2
}
declare enum MouseButton {
    NoButton = 0,
    LeftButton = 1,
    RightButton = 2,
    MiddleButton = 3,
    BackButton = 4,
    ForwardButton = 5,
    End = 6
}
declare enum AmbientMode {
    Skybox = 0,
    Trilight = 1,
    Flat = 3,
    Custom = 4
}
declare enum NetInstanceState {
    Invalid = 0,
    Local = 1,
    Remote = 2
}
declare enum NetOrigin {
    Unknow = 0,
    Client = 1,
    Server = 2,
    ServerConversation = 3
}
declare enum PSEmitterVelocityMode {
    Transform = 0,
    Rigidbody = 1,
    Custom = 2
}
declare enum GUISpriteSequenceEvent {
    Click = 0,
    ScrollWheel = 5,
    PointerDrag = 6,
    PointerStationary = 7,
    KeyDown = 14,
    KeyPress = 15,
    KeyUp = 16,
    PointerEnter = 1,
    PointerLeave = 2,
    PointerUp = 3,
    PointerDown = 4,
    Selected = 8,
    LostSelected = 9,
    Show = 10,
    Hidden = 11,
    Enabled = 12,
    Disabled = 13,
    Start = 18,
    Updated = 19,
    Destruction = 20,
    AddChild = 21,
    RemoveChild = 22,
    LayoutResize = 23
}
declare enum CapsuleCollderDirection {
    X_Axis = 0,
    Y_Axis = 1,
    Z_Axis = 2
}
declare enum PrimitiveType {
    Sphere = 0,
    Capsule = 1,
    Cylinder = 2,
    Cube = 3,
    Plane = 4,
    Quad = 5
}
declare enum PhysicMaterialCombine {
    Average = 0,
    Multiply = 2,
    Minimum = 1,
    Maximum = 3
}
declare enum ImageType {
    Simple = 0,
    Sliced = 1,
    Tiled = 2,
    Filled = 3
}
declare enum AnimationPlayMode {
    StopSameLayer = 0,
    StopAll = 1
}
declare enum NavMeshObstacleShape {
    Cylinder = 0,
    Box = 1
}
declare enum AnisoLevel {
    None = 1,
    x2 = 2,
    x4 = 4,
    x8 = 8,
    x16 = 16
}
declare enum TextureDimension {
    None = 0,
    Any = 1,
    Tex2D = 2,
    Tex3D = 3,
    Cube = 4,
    Tex2DArray = 5,
    CubeArray = 6
}
declare enum TextureWrapMode {
    Repeat = 0,
    Clamp = 1,
    Mirror = 2
}
declare enum FilterMode {
    Point = 0,
    Bilinear = 1,
    Trilinear = 2
}
declare enum CameraType {
    None = 0,
    Game = 1,
    SceneView = 2,
    Preview = 4,
    All = 7
}
declare enum SliderDirection {
    LeftToRight = 0,
    RightToLeft = 1,
    BottomToTop = 2,
    TopToBottom = 3
}
declare enum TextBoxEvent {
    ValidityChanged = 33,
    ValidityChangedState = 34,
    PointerEnter = 1,
    PointerLeave = 2,
    PointerUp = 3,
    PointerDown = 4,
    Selected = 8,
    LostSelected = 9,
    Show = 10,
    Hidden = 11,
    Enabled = 12,
    Disabled = 13,
    Start = 18,
    Updated = 19,
    Destruction = 20,
    AddChild = 21,
    RemoveChild = 22,
    LayoutResize = 23
}
declare enum AnimationBlendMode {
    Blend = 0,
    Additive = 1
}
declare enum CascadeShadowType {
    NoSplit = 0,
    TwoSplit = 1,
    FourSplit = 2
}
declare enum DepthTextureMode {
    None = 0,
    Depth = 1,
    DepthNormals = 2,
    MotionVectors = 4
}
declare enum QueryTriggerInteraction {
    UseGlobal = 0,
    Ignore = 1,
    Hit = 2
}
declare enum DepthType {
    None = 0,
    Depth16 = 1,
    Depth24 = 2,
    Depth24Stencil8 = 3
}
declare enum FogMode {
    Linear = 1,
    Exponential = 2,
    ExponentialSquared = 3
}
declare enum GraphicsFormat {
    None = 0,
    R8_SRGB = 1,
    R8G8_SRGB = 2,
    R8G8B8_SRGB = 3,
    R8G8B8A8_SRGB = 4,
    R8_UNorm = 5,
    R8G8_UNorm = 6,
    R8G8B8_UNorm = 7,
    R8G8B8A8_UNorm = 8,
    R8_SNorm = 9,
    R8G8_SNorm = 10,
    R8G8B8_SNorm = 11,
    R8G8B8A8_SNorm = 12,
    R8_UInt = 13,
    R8G8_UInt = 14,
    R8G8B8_UInt = 15,
    R8G8B8A8_UInt = 16,
    R8_SInt = 17,
    R8G8_SInt = 18,
    R8G8B8_SInt = 19,
    R8G8B8A8_SInt = 20,
    R16_UNorm = 21,
    R16G16_UNorm = 22,
    R16G16B16_UNorm = 23,
    R16G16B16A16_UNorm = 24,
    R16_SNorm = 25,
    R16G16_SNorm = 26,
    R16G16B16_SNorm = 27,
    R16G16B16A16_SNorm = 28,
    R16_UInt = 29,
    R16G16_UInt = 30,
    R16G16B16_UInt = 31,
    R16G16B16A16_UInt = 32,
    R16_SInt = 33,
    R16G16_SInt = 34,
    R16G16B16_SInt = 35,
    R16G16B16A16_SInt = 36,
    R32_UInt = 37,
    R32G32_UInt = 38,
    R32G32B32_UInt = 39,
    R32G32B32A32_UInt = 40,
    R32_SInt = 41,
    R32G32_SInt = 42,
    R32G32B32_SInt = 43,
    R32G32B32A32_SInt = 44,
    R16_SFloat = 45,
    R16G16_SFloat = 46,
    R16G16B16_SFloat = 47,
    R16G16B16A16_SFloat = 48,
    R32_SFloat = 49,
    R32G32_SFloat = 50,
    R32G32B32_SFloat = 51,
    R32G32B32A32_SFloat = 52,
    B8G8R8_SRGB = 56,
    B8G8R8A8_SRGB = 57,
    B8G8R8_UNorm = 58,
    B8G8R8A8_UNorm = 59,
    B8G8R8_SNorm = 60,
    B8G8R8A8_SNorm = 61,
    B8G8R8_UInt = 62,
    B8G8R8A8_UInt = 63,
    B8G8R8_SInt = 64,
    B8G8R8A8_SInt = 65,
    R4G4B4A4_UNormPack16 = 66,
    B4G4R4A4_UNormPack16 = 67,
    R5G6B5_UNormPack16 = 68,
    B5G6R5_UNormPack16 = 69,
    R5G5B5A1_UNormPack16 = 70,
    B5G5R5A1_UNormPack16 = 71,
    A1R5G5B5_UNormPack16 = 72,
    E5B9G9R9_UFloatPack32 = 73,
    B10G11R11_UFloatPack32 = 74,
    A2B10G10R10_UNormPack32 = 75,
    A2B10G10R10_UIntPack32 = 76,
    A2B10G10R10_SIntPack32 = 77,
    A2R10G10B10_UNormPack32 = 78,
    A2R10G10B10_UIntPack32 = 79,
    A2R10G10B10_SIntPack32 = 80,
    A2R10G10B10_XRSRGBPack32 = 81,
    A2R10G10B10_XRUNormPack32 = 82,
    R10G10B10_XRSRGBPack32 = 83,
    R10G10B10_XRUNormPack32 = 84,
    A10R10G10B10_XRSRGBPack32 = 85,
    A10R10G10B10_XRUNormPack32 = 86,
    D16_UNorm = 90,
    D24_UNorm = 91,
    D24_UNorm_S8_UInt = 92,
    D32_SFloat = 93,
    D32_SFloat_S8_UInt = 94,
    S8_UInt = 95,
    RGB_DXT1_SRGB = 96,
    RGBA_DXT1_SRGB = 96,
    RGB_DXT1_UNorm = 97,
    RGBA_DXT1_UNorm = 97,
    RGBA_DXT3_SRGB = 98,
    RGBA_DXT3_UNorm = 99,
    RGBA_DXT5_SRGB = 100,
    RGBA_DXT5_UNorm = 101,
    R_BC4_UNorm = 102,
    R_BC4_SNorm = 103,
    RG_BC5_UNorm = 104,
    RG_BC5_SNorm = 105,
    RGB_BC6H_UFloat = 106,
    RGB_BC6H_SFloat = 107,
    RGBA_BC7_SRGB = 108,
    RGBA_BC7_UNorm = 109,
    RGB_PVRTC_2Bpp_SRGB = 110,
    RGB_PVRTC_2Bpp_UNorm = 111,
    RGB_PVRTC_4Bpp_SRGB = 112,
    RGB_PVRTC_4Bpp_UNorm = 113,
    RGBA_PVRTC_2Bpp_SRGB = 114,
    RGBA_PVRTC_2Bpp_UNorm = 115,
    RGBA_PVRTC_4Bpp_SRGB = 116,
    RGBA_PVRTC_4Bpp_UNorm = 117,
    RGB_ETC_UNorm = 118,
    RGB_ETC2_SRGB = 119,
    RGB_ETC2_UNorm = 120,
    RGB_A1_ETC2_SRGB = 121,
    RGB_A1_ETC2_UNorm = 122,
    RGBA_ETC2_SRGB = 123,
    RGBA_ETC2_UNorm = 124,
    R_EAC_UNorm = 125,
    R_EAC_SNorm = 126,
    RG_EAC_UNorm = 127,
    RG_EAC_SNorm = 128,
    RGBA_ASTC4X4_SRGB = 129,
    RGBA_ASTC4X4_UNorm = 130,
    RGBA_ASTC5X5_SRGB = 131,
    RGBA_ASTC5X5_UNorm = 132,
    RGBA_ASTC6X6_SRGB = 133,
    RGBA_ASTC6X6_UNorm = 134,
    RGBA_ASTC8X8_SRGB = 135,
    RGBA_ASTC8X8_UNorm = 136,
    RGBA_ASTC10X10_SRGB = 137,
    RGBA_ASTC10X10_UNorm = 138,
    RGBA_ASTC12X12_SRGB = 139,
    RGBA_ASTC12X12_UNorm = 140,
    YUV2 = 141,
    DepthAuto = 142,
    ShadowAuto = 143,
    VideoAuto = 144,
    RGBA_ASTC4X4_UFloat = 145,
    RGBA_ASTC5X5_UFloat = 146,
    RGBA_ASTC6X6_UFloat = 147,
    RGBA_ASTC8X8_UFloat = 148,
    RGBA_ASTC10X10_UFloat = 149,
    RGBA_ASTC12X12_UFloat = 150,
    A8R8G8B8_UNorm = 151
}
declare enum LightType {
    Spot = 0,
    Directional = 1,
    Point = 2,
    Area = 3
}
declare enum RenderTextureFormat {
    RGBA32 = 0,
    Depth = 1,
    RGBAHalf = 2,
    Shadowmap = 3,
    RGB565 = 4,
    RGBA4444 = 5,
    RGBA5551 = 6,
    Default = 7,
    ARGB2101010 = 8,
    DefaultHDR = 9,
    ARGB64 = 10,
    ARGBFloat = 11,
    RGFloat = 12,
    RGHalf = 13,
    RFloat = 14,
    RHalf = 15,
    R8 = 16,
    RGBAInt = 17,
    RGInt = 18,
    RInt = 19,
    BRG101111Float = 20,
    RGBAUShort = 21,
    R16 = 22,
    ShadowmapNoReverse = 23,
    Max = 24
}
declare enum LineAlignment {
    View = 0,
    TransformZ = 1
}
declare enum PSShapeType {
    Sphere = 0,
    Hemisphere = 1,
    Cone = 2,
    Donut = 3,
    Box = 4,
    Mesh = 5,
    Circle = 6,
    Edge = 7,
    Rectangle = 8,
    MaxNum = 9
}
declare enum ShadowCastingMode {
    Off = 0,
    On = 1,
    TwoSided = 2
}
declare enum ShadowResolutionType {
    Low = 512,
    Medium = 1024,
    High = 2048,
    Ultra = 4096
}
declare enum ShadowType {
    Disable = 0,
    Hard = 1,
    Soft = 2
}
declare enum SkyboxType {
    Cubemap = 0,
    Procedual = 1
}
declare enum AssetType {
    Unknow = 0,
    Prefab = 1,
    Material = 2,
    Mesh = 3,
    Texture = 4,
    Texture2D = 5,
    CubeMap = 6,
    PhysicsMaterial = 7,
    Shader = 8,
    Scene = 9,
    UI = 10,
    AudioClip = 11,
    AnimationClip = 12,
    AnimationMask = 13,
    AnimatorData = 14,
    Skeleton = 15,
    Atlas = 17,
    Font = 19,
    LightProbe = 20,
    ShaderHeader = 21,
    CSV = 22,
    NmAPISetting = 24,
    NmMesh = 25,
    Sprite = 26,
    SpriteSequence = 27,
    Text = 29,
    Terrain = 30
}
declare enum AudioClipLoadType {
    DecompressOnLoad = 0,
    CompressedInMemory = 1,
    Streaming = 2
}
declare enum AudioPlayState {
    AUDIO_IDLE = 0,
    AUDIO_PLAYING = 1,
    AUDIO_PAUSING = 2
}
declare enum MaskType {
    Image = 0,
    Polygon = 1
}
declare enum LayoutFillStartType {
    TopLeft = 0,
    TopRight = 1,
    BottomLeft = 2,
    BottomRight = 3
}
declare enum EventType {
    None = 0,
    PointerDown = 1,
    PointerUp = 2,
    PointerDrag = 4,
    PointerEnter = 8,
    PointerLeave = 16,
    PointerStationary = 32,
    ScrollWheel = 64,
    KeyDown = 128,
    KeyUp = 256
}
declare enum CanvasMode {
    Screen = 0,
    Scene = 1
}
declare enum BehaviorUpdateInterval {
    Frame = 0,
    SpecifiedSecond = 1,
    Manual = 2
}
declare enum PSSimulationSpace {
    Local = 0,
    World = 1,
    Custom = 2
}
declare enum ScrollbarEvent {
    Scroll = 27,
    PointerEnter = 1,
    PointerLeave = 2,
    PointerUp = 3,
    PointerDown = 4,
    Selected = 8,
    LostSelected = 9,
    Show = 10,
    Hidden = 11,
    Enabled = 12,
    Disabled = 13,
    Start = 18,
    Updated = 19,
    Destruction = 20,
    AddChild = 21,
    RemoveChild = 22,
    LayoutResize = 23
}
declare enum AnimationCurveType {
    None = 0,
    Bool = 1,
    Float = 10,
    Vector2 = 12,
    Vector3 = 13,
    Vector4 = 14,
    Quaternion = 17,
    Color = 20
}
declare enum ButtonEvent {
    Click = 0,
    PointerDrag = 6,
    PointerStationary = 7,
    PointerEnter = 1,
    PointerLeave = 2,
    PointerUp = 3,
    PointerDown = 4,
    Selected = 8,
    LostSelected = 9,
    Show = 10,
    Hidden = 11,
    Enabled = 12,
    Disabled = 13,
    Start = 18,
    Updated = 19,
    Destruction = 20,
    AddChild = 21,
    RemoveChild = 22,
    LayoutResize = 23
}
declare enum ContentLimit {
    Standard = 0,
    IntegerNumber = 1,
    DecimalNumber = 2,
    Alphanumeric = 3,
    Password = 4,
    Pin = 5
}
declare enum ControlBaseEvent {
    Show = 10,
    Hidden = 11,
    Enabled = 12,
    Disabled = 13,
    Start = 18,
    Updated = 19,
    Destruction = 20,
    AddChild = 21,
    RemoveChild = 22,
    LayoutResize = 23
}
declare enum ControlType {
    Form = 0,
    Canvas = 1,
    Image = 2,
    Text = 3,
    Button = 4,
    Dropdown = 5,
    Slider = 6,
    Scrollbar = 7,
    ScrollView = 8,
    TextBox = 9,
    Toggle = 10,
    Default = 11,
    Mask = 12,
    SpriteSequence = 13
}
declare enum LineTextureMode {
    Stretch = 0,
    Tile = 1,
    DistributePerSegment = 2,
    RepeatPerSegment = 3
}
declare enum FillMethod {
    Horizontal = 0,
    Vertical = 1,
    Radial = 2
}
declare enum FillStartPoint {
    Left = 0,
    Top = 1,
    Right = 2,
    Bottom = 3
}
declare enum ContractState {
    Exescute = 0,
    Receive = 1,
    Respond = 2
}
declare enum FontStyle {
    Nomal = 0,
    Bold = 1,
    Italic = 2,
    Underline = 4,
    Strikeout = 8,
    BoldAndItalic = 3
}
declare enum ImageEvent {
    Click = 0,
    ScrollWheel = 5,
    PointerDrag = 6,
    PointerStationary = 7,
    KeyDown = 14,
    KeyPress = 15,
    KeyUp = 16,
    PointerEnter = 1,
    PointerLeave = 2,
    PointerUp = 3,
    PointerDown = 4,
    Selected = 8,
    LostSelected = 9,
    Show = 10,
    Hidden = 11,
    Enabled = 12,
    Disabled = 13,
    Start = 18,
    Updated = 19,
    Destruction = 20,
    AddChild = 21,
    RemoveChild = 22,
    LayoutResize = 23
}
declare enum InputReturnType {
    Default = 0,
    Done = 1,
    Send = 2,
    Search = 3,
    Go = 4
}
declare enum LayoutAlignmentType {
    TopLeft = 0,
    TopCenter = 1,
    TopRight = 2,
    MiddleLeft = 3,
    MiddleCenter = 4,
    MiddleRight = 5,
    BottomLeft = 6,
    BottomCenter = 7,
    BottomRight = 8
}
declare enum LayoutFillDirectionType {
    Horizontal = 0,
    Vertical = 1
}
declare enum LayoutGroupType {
    None = 0,
    Horizontal = 1,
    Vertical = 2,
    Grid = 3
}
declare enum MovementType {
    Unrestricted = 0,
    Elastic = 1,
    Clamped = 2
}
declare enum TweenEaseType {
    LinearIn = 0,
    LinearOut = 1,
    LinearInOut = 2,
    LinearOutIn = 3,
    QuadIn = 4,
    QuadOut = 5,
    QuadInOut = 6,
    QuadOutIn = 7,
    CubicIn = 8,
    CubicOut = 9,
    CubicInOut = 10,
    CubicOutIn = 11,
    QuartIn = 12,
    QuartOut = 13,
    QuartInOut = 14,
    QuartOutIn = 15,
    QuintIn = 16,
    QuintOut = 17,
    QuintInOut = 18,
    QuintOutIn = 19,
    SineIn = 20,
    SineOut = 21,
    SineInOut = 22,
    SineOutIn = 23,
    ExpoIn = 24,
    ExpoOut = 25,
    ExpoInOut = 26,
    ExpoOutIn = 27,
    CircIn = 28,
    CircOut = 29,
    CircInOut = 30,
    CircOutIn = 31,
    ElasticIn = 32,
    ElasticOut = 33,
    ElasticInOut = 34,
    ElasticOutIn = 35,
    BackIn = 36,
    BackOut = 37,
    BackInOut = 38,
    BackOutIn = 39,
    BounceIn = 40,
    BounceOut = 41,
    BounceInOut = 42,
    BounceOutIn = 43,
    End = 44
}
declare enum ScrollViewEvent {
    ContentMove = 28,
    PullDown = 29,
    PullUp = 30,
    PullLeft = 31,
    PullRight = 32,
    Show = 10,
    Hidden = 11,
    Enabled = 12,
    Disabled = 13,
    Start = 18,
    Updated = 19,
    Destruction = 20,
    AddChild = 21,
    RemoveChild = 22,
    LayoutResize = 23
}
declare enum SliderEvent {
    ValueChanged = 26,
    PointerEnter = 1,
    PointerLeave = 2,
    PointerUp = 3,
    PointerDown = 4,
    Selected = 8,
    LostSelected = 9,
    Show = 10,
    Hidden = 11,
    Enabled = 12,
    Disabled = 13,
    Start = 18,
    Updated = 19,
    Destruction = 20,
    AddChild = 21,
    RemoveChild = 22,
    LayoutResize = 23
}
declare enum SliderType {
    Filled = 0,
    Sliced = 1
}
declare enum TextAnchor {
    UpperLeft = 0,
    UpperCenter = 1,
    UpperRight = 2,
    MiddleLeft = 3,
    MiddleCenter = 4,
    MiddleRight = 5,
    LowerLeft = 6,
    LowerCenter = 7,
    LowerRight = 8
}
declare enum TextFitter {
    Unconstrained = 0,
    PreferredSize = 1
}
declare enum TextHAlignment {
    Left = 0,
    Center = 1,
    Right = 2
}
declare enum TextVAlignment {
    Upper = 0,
    Middle = 1,
    Lower = 2
}
declare enum ToggleEvent {
    StateChanged = 35,
    PointerEnter = 1,
    PointerLeave = 2,
    PointerUp = 3,
    PointerDown = 4,
    Selected = 8,
    LostSelected = 9,
    Show = 10,
    Hidden = 11,
    Enabled = 12,
    Disabled = 13,
    Start = 18,
    Updated = 19,
    Destruction = 20,
    AddChild = 21,
    RemoveChild = 22,
    LayoutResize = 23
}
declare enum TransitionMode {
    None = 0,
    ColorTint = 1,
    SpriteSwap = 2
}
declare enum AdditionalLightSliceResolutionType {
    Low = 64,
    Medium = 128,
    High = 256,
    Ultra = 512
}
declare enum AnimationCullMode {
    None = 0,
    Transforms = 1,
    Complete = 2
}
declare enum BendModifier {
    BendRoot = 0,
    BendTarget = 1,
    BendParent = 2,
    BendGoal = 3
}
declare enum WrapMode {
    Default = 0,
    Clamp = 1,
    Repeat = 2,
    PingPong = 3
}
declare enum NavMeshPathStatus {
    PathComplete = 0,
    PathPartial = 1,
    PathInvalid = 2
}
declare enum KeyCode {
    None = 0,
    Backspace = 8,
    Tab = 9,
    Clear = 12,
    Enter = 13,
    Pause = 19,
    ESC = 27,
    Space = 32,
    Exclaim = 33,
    DoubleQuote = 34,
    Hash = 35,
    Dollar = 36,
    Percent = 37,
    Ampersand = 38,
    Quote = 39,
    LeftParen = 40,
    RightParen = 41,
    Asterisk = 42,
    Plus = 43,
    Comma = 44,
    Minus = 45,
    Period = 46,
    Slash = 47,
    Alpha0 = 48,
    Alpha1 = 49,
    Alpha2 = 50,
    Alpha3 = 51,
    Alpha4 = 52,
    Alpha5 = 53,
    Alpha6 = 54,
    Alpha7 = 55,
    Alpha8 = 56,
    Alpha9 = 57,
    Colon = 58,
    Semicolon = 59,
    Less = 60,
    Equals = 61,
    Greater = 62,
    Question = 63,
    At = 64,
    LeftBracket = 91,
    Backslash = 92,
    RightBracket = 93,
    Caret = 94,
    Underscore = 95,
    BackQuote = 96,
    A = 97,
    B = 98,
    C = 99,
    D = 100,
    E = 101,
    F = 102,
    G = 103,
    H = 104,
    I = 105,
    J = 106,
    K = 107,
    L = 108,
    M = 109,
    N = 110,
    O = 111,
    P = 112,
    Q = 113,
    R = 114,
    S = 115,
    T = 116,
    U = 117,
    V = 118,
    W = 119,
    X = 120,
    Y = 121,
    Z = 122,
    LeftCurlyBracket = 123,
    Pipe = 124,
    RightCurlyBracket = 125,
    Tilde = 126,
    Delete = 127,
    Keypad0 = 256,
    Keypad1 = 257,
    Keypad2 = 258,
    Keypad3 = 259,
    Keypad4 = 260,
    Keypad5 = 261,
    Keypad6 = 262,
    Keypad7 = 263,
    Keypad8 = 264,
    Keypad9 = 265,
    KeypadPeriod = 266,
    KeypadDivide = 267,
    KeypadMultiply = 268,
    KeypadMinus = 269,
    KeypadPlus = 270,
    KeypadEnter = 271,
    KeypadEquals = 272,
    UpArrow = 273,
    DownArrow = 274,
    RightArrow = 275,
    LeftArrow = 276,
    Insert = 277,
    Home = 278,
    End = 279,
    PageUp = 280,
    PageDown = 281,
    F1 = 282,
    F2 = 283,
    F3 = 284,
    F4 = 285,
    F5 = 286,
    F6 = 287,
    F7 = 288,
    F8 = 289,
    F9 = 290,
    F10 = 291,
    F11 = 292,
    F12 = 293,
    F13 = 294,
    F14 = 295,
    F15 = 296,
    Numlock = 300,
    CapsLock = 301,
    ScrollLock = 302,
    RightShift = 303,
    LeftShift = 304,
    RightControl = 305,
    LeftControl = 306,
    RightAlt = 307,
    LeftAlt = 308,
    RightCommand = 309,
    RightApple = 309,
    LeftCommand = 310,
    LeftApple = 310,
    LeftWindows = 311,
    RightWindows = 312,
    AltGr = 313,
    Help = 315,
    Print = 316,
    SysReq = 317,
    Break = 318,
    Menu = 319,
    JoystickButton0 = 330,
    JoystickButton1 = 331,
    JoystickButton2 = 332,
    JoystickButton3 = 333,
    JoystickButton4 = 334,
    JoystickButton5 = 335,
    JoystickButton6 = 336,
    JoystickButton7 = 337,
    JoystickButton8 = 338,
    JoystickButton9 = 339,
    JoystickButton10 = 340,
    JoystickButton11 = 341,
    JoystickButton12 = 342,
    JoystickButton13 = 343,
    JoystickButton14 = 344,
    JoystickButton15 = 345,
    JoystickButton16 = 346,
    JoystickButton17 = 347,
    JoystickButton18 = 348,
    JoystickButton19 = 349,
    Joystick1Button0 = 350,
    Joystick1Button1 = 351,
    Joystick1Button2 = 352,
    Joystick1Button3 = 353,
    Joystick1Button4 = 354,
    Joystick1Button5 = 355,
    Joystick1Button6 = 356,
    Joystick1Button7 = 357,
    Joystick1Button8 = 358,
    Joystick1Button9 = 359,
    Joystick1Button10 = 360,
    Joystick1Button11 = 361,
    Joystick1Button12 = 362,
    Joystick1Button13 = 363,
    Joystick1Button14 = 364,
    Joystick1Button15 = 365,
    Joystick1Button16 = 366,
    Joystick1Button17 = 367,
    Joystick1Button18 = 368,
    Joystick1Button19 = 369,
    Joystick2Button0 = 370,
    Joystick2Button1 = 371,
    Joystick2Button2 = 372,
    Joystick2Button3 = 373,
    Joystick2Button4 = 374,
    Joystick2Button5 = 375,
    Joystick2Button6 = 376,
    Joystick2Button7 = 377,
    Joystick2Button8 = 378,
    Joystick2Button9 = 379,
    Joystick2Button10 = 380,
    Joystick2Button11 = 381,
    Joystick2Button12 = 382,
    Joystick2Button13 = 383,
    Joystick2Button14 = 384,
    Joystick2Button15 = 385,
    Joystick2Button16 = 386,
    Joystick2Button17 = 387,
    Joystick2Button18 = 388,
    Joystick2Button19 = 389,
    Joystick3Button0 = 390,
    Joystick3Button1 = 391,
    Joystick3Button2 = 392,
    Joystick3Button3 = 393,
    Joystick3Button4 = 394,
    Joystick3Button5 = 395,
    Joystick3Button6 = 396,
    Joystick3Button7 = 397,
    Joystick3Button8 = 398,
    Joystick3Button9 = 399,
    Joystick3Button10 = 400,
    Joystick3Button11 = 401,
    Joystick3Button12 = 402,
    Joystick3Button13 = 403,
    Joystick3Button14 = 404,
    Joystick3Button15 = 405,
    Joystick3Button16 = 406,
    Joystick3Button17 = 407,
    Joystick3Button18 = 408,
    Joystick3Button19 = 409,
    Joystick4Button0 = 410,
    Joystick4Button1 = 411,
    Joystick4Button2 = 412,
    Joystick4Button3 = 413,
    Joystick4Button4 = 414,
    Joystick4Button5 = 415,
    Joystick4Button6 = 416,
    Joystick4Button7 = 417,
    Joystick4Button8 = 418,
    Joystick4Button9 = 419,
    Joystick4Button10 = 420,
    Joystick4Button11 = 421,
    Joystick4Button12 = 422,
    Joystick4Button13 = 423,
    Joystick4Button14 = 424,
    Joystick4Button15 = 425,
    Joystick4Button16 = 426,
    Joystick4Button17 = 427,
    Joystick4Button18 = 428,
    Joystick4Button19 = 429,
    Joystick5Button0 = 430,
    Joystick5Button1 = 431,
    Joystick5Button2 = 432,
    Joystick5Button3 = 433,
    Joystick5Button4 = 434,
    Joystick5Button5 = 435,
    Joystick5Button6 = 436,
    Joystick5Button7 = 437,
    Joystick5Button8 = 438,
    Joystick5Button9 = 439,
    Joystick5Button10 = 440,
    Joystick5Button11 = 441,
    Joystick5Button12 = 442,
    Joystick5Button13 = 443,
    Joystick5Button14 = 444,
    Joystick5Button15 = 445,
    Joystick5Button16 = 446,
    Joystick5Button17 = 447,
    Joystick5Button18 = 448,
    Joystick5Button19 = 449,
    Joystick6Button0 = 450,
    Joystick6Button1 = 451,
    Joystick6Button2 = 452,
    Joystick6Button3 = 453,
    Joystick6Button4 = 454,
    Joystick6Button5 = 455,
    Joystick6Button6 = 456,
    Joystick6Button7 = 457,
    Joystick6Button8 = 458,
    Joystick6Button9 = 459,
    Joystick6Button10 = 460,
    Joystick6Button11 = 461,
    Joystick6Button12 = 462,
    Joystick6Button13 = 463,
    Joystick6Button14 = 464,
    Joystick6Button15 = 465,
    Joystick6Button16 = 466,
    Joystick6Button17 = 467,
    Joystick6Button18 = 468,
    Joystick6Button19 = 469,
    Joystick7Button0 = 470,
    Joystick7Button1 = 471,
    Joystick7Button2 = 472,
    Joystick7Button3 = 473,
    Joystick7Button4 = 474,
    Joystick7Button5 = 475,
    Joystick7Button6 = 476,
    Joystick7Button7 = 477,
    Joystick7Button8 = 478,
    Joystick7Button9 = 479,
    Joystick7Button10 = 480,
    Joystick7Button11 = 481,
    Joystick7Button12 = 482,
    Joystick7Button13 = 483,
    Joystick7Button14 = 484,
    Joystick7Button15 = 485,
    Joystick7Button16 = 486,
    Joystick7Button17 = 487,
    Joystick7Button18 = 488,
    Joystick7Button19 = 489
}
declare enum CollisionFlags {
    None = 0,
    Sides = 1,
    Above = 2,
    Below = 4
}
declare enum BehaviorLogType {
    Log = 0,
    Warning = 1,
    Error = 2
}
declare enum AnimationQueueMode {
    CompleteOthers = 0,
    PlayNow = 1
}
declare enum GamePadAxis {
    LeftTrigger = 0,
    RightTrigger = 1,
    LeftJoyStickHorizontal = 2,
    LeftJoyStickVertical = 3,
    RightJoyStickHorizontal = 4,
    RightJoyStickVertical = 5
}
declare enum PSRenderMode {
    Billboard = 0,
    Stretch3D = 1,
    BillboardFixedHorizontal = 2,
    BillboardFixedVertical = 3,
    Mesh = 4,
    None = 5
}
declare enum PSSortMode {
    None = 0,
    ByDistance = 1,
    OldestInFront = 2,
    YounestInFront = 3
}
declare enum BehaviorAbortType {
    None = 0,
    Self = 1,
    LowerPriority = 2,
    Both = 3
}
declare enum PSForceSpace {
    Local = 0,
    World = 1
}
declare enum BehaviorTaskStatus {
    Inactive = 0,
    Failure = 1,
    Success = 2,
    Running = 3
}
declare enum ObstacleAvoidanceType {
    NoObstacleAvoidance = 0,
    LowQualityObstacleAvoidance = 1,
    MedQualityObstacleAvoidance = 2,
    GoodQualityObstacleAvoidance = 3,
    HighQualityObstacleAvoidance = 4
}
declare enum OffMeshLinkType {
    LinkTypeManual = 0,
    LinkTypeDropDown = 1,
    LinkTypeJumpAcross = 2
}
declare enum RigidbodyConstraints {
    None = 0,
    FreezePositionX = 2,
    FreezePositionY = 4,
    FreezePositionZ = 8,
    FreezePosition = 14,
    FreezeRotationX = 16,
    FreezeRotationY = 32,
    FreezeRotationZ = 64,
    FreezeRotation = 112,
    FreezeAll = 126
}
declare enum RigidbodyInterpolation {
    None = 0,
    Interpolate = 1,
    Extrapolate = 2
}
declare enum CollisionDetectionMode {
    Discrete = 0,
    Continuous = 1,
    ContinuousDynamic = 2,
    ContinuousSpeculative = 3
}
declare enum ForceMode {
    Force = 0,
    Impulse = 1,
    VelocityChange = 2,
    Acceleration = 3
}
declare enum UpdateMode {
    Normal = 0,
    AnimatePhysics = 1,
    UnscaledTime = 2,
    Default = 3
}
declare enum PSCurveMode {
    Scalar = 0,
    TwoConstants = 1,
    Curve = 2,
    TwoCurves = 3
}
declare enum PSGradientMode {
    Color = 0,
    TwoColor = 1,
    Gradient = 2,
    TwoGradient = 3,
    Random = 4
}
declare enum PSRenderSpace {
    View = 0,
    World = 1,
    Local = 2,
    Facing = 3,
    Velocity = 4
}
declare enum PSScalingMode {
    Hierarchy = 0,
    Local = 1,
    Shape = 2
}
declare function Assert(condition: boolean, ...msg: any[]): void;
declare enum PSShapeStyle {
    Volume = 0,
    Surface = 1,
    Base = 2,
    Shell = 1,
    Edge = 2,
    MaxNum = 3
}
declare enum PSStopBehavior {
    StopEmittingAndClear = 0,
    StopEmitting = 1
}
declare enum PSVelocityInheritMode {
    Initial = 0,
    Current = 1
}
declare enum PSVelocitySpace {
    Local = 0,
    World = 1
}
declare enum PostEffectDefine {
    FastAA = 0,
    Bloom = 1,
    Vignette = 2,
    ColorGrading = 3
}
declare enum TweenLoopType {
    Restart = 0,
    Yoyo = 1
}
declare enum FillOrigin {
    Center = 0,
    Left = 1,
    Top = 2,
    Right = 3,
    Bottom = 4,
    LeftTop = 5,
    RightTop = 6,
    RightBottom = 7,
    LeftBottom = 8
}
declare enum AntiAliasingLevel {
    None = 0,
    x2 = 2,
    x4 = 4,
    x8 = 8
}
declare enum PlayState {
    Stopped = 0,
    Playing = 1,
    Paused = 2
}
declare enum EngineOrigin {
    Unknown = 0,
    Client = 1,
    Server = 2,
    Editor = 3
}
declare enum DropdownEvent {
    Click = 0,
    SelectChanged = 25,
    PointerEnter = 1,
    PointerLeave = 2,
    PointerUp = 3,
    PointerDown = 4,
    Selected = 8,
    LostSelected = 9,
    Show = 10,
    Hidden = 11,
    Enabled = 12,
    Disabled = 13,
    Start = 18,
    Updated = 19,
    Destruction = 20,
    AddChild = 21,
    RemoveChild = 22,
    LayoutResize = 23
}
declare enum TouchMask {
    UI = 1
}
//# sourceMappingURL=Assembly.d.ts.map