var __extends = (this && this.__extends) || (function () {
    var extendStatics = function (d, b) {
        extendStatics = Object.setPrototypeOf ||
            ({ __proto__: [] } instanceof Array && function (d, b) { d.__proto__ = b; }) ||
            function (d, b) { for (var p in b) if (Object.prototype.hasOwnProperty.call(b, p)) d[p] = b[p]; };
        return extendStatics(d, b);
    };
    return function (d, b) {
        if (typeof b !== "function" && b !== null)
            throw new TypeError("Class extends value " + String(b) + " is not a constructor or null");
        extendStatics(d, b);
        function __() { this.constructor = d; }
        d.prototype = b === null ? Object.create(b) : (__.prototype = b.prototype, new __());
    };
})();
var __decorate = (this && this.__decorate) || function (decorators, target, key, desc) {
    var c = arguments.length, r = c < 3 ? target : desc === null ? desc = Object.getOwnPropertyDescriptor(target, key) : desc, d;
    if (typeof Reflect === "object" && typeof Reflect.decorate === "function") r = Reflect.decorate(decorators, target, key, desc);
    else for (var i = decorators.length - 1; i >= 0; i--) if (d = decorators[i]) r = (c < 3 ? d(r) : c > 3 ? d(target, key, r) : d(target, key)) || r;
    return c > 3 && r && Object.defineProperty(target, key, r), r;
};
var __read = (this && this.__read) || function (o, n) {
    var m = typeof Symbol === "function" && o[Symbol.iterator];
    if (!m) return o;
    var i = m.call(o), r, ar = [], e;
    try {
        while ((n === void 0 || n-- > 0) && !(r = i.next()).done) ar.push(r.value);
    }
    catch (error) { e = { error: error }; }
    finally {
        try {
            if (r && !r.done && (m = i["return"])) m.call(i);
        }
        finally { if (e) throw e.error; }
    }
    return ar;
};
var __spreadArray = (this && this.__spreadArray) || function (to, from, pack) {
    if (pack || arguments.length === 2) for (var i = 0, l = from.length, ar; i < l; i++) {
        if (ar || !(i in from)) {
            if (!ar) ar = Array.prototype.slice.call(from, 0, i);
            ar[i] = from[i];
        }
    }
    return to.concat(ar || Array.prototype.slice.call(from));
};
var CameraController = (function (_super) {
    __extends(CameraController, _super);
    function CameraController() {
        var _this = _super.apply(this, __spreadArray([], __read(arguments), false)) || this;
        _this.MaxDistance = 5;
        _this.MinDistance = 1;
        _this.MaxRotateAngle = 60;
        _this.MinRotateAngle = -60;
        _this.RotateSensitivity = 30;
        _this.Offset = 0.1;
        _this.SmoothSpeed = 0.1;
        _this._viewHorizontal = 0;
        _this._viewVertical = 0;
        _this._yaw = 0;
        _this._pitch = 0;
        _this._outDistance = 2;
        return _this;
    }
    CameraController.prototype.OnLateUpdate = function () {
        this.CameraMove();
    };
    CameraController.prototype.CameraMove = function () {
        this._yaw += this._viewHorizontal;
        this._pitch -= this._viewVertical;
        this._pitch = Mathf.Clamp(this._pitch, this.MinRotateAngle, this.MaxRotateAngle);
        var hit = new RaycastHit();
        var ray = new Ray(this.CameraFollow.transform.position, this.transform.position.Sub(this.CameraFollow.transform.position));
        hit = Physics.Raycast(ray, this.MaxDistance, this.HitLayerMask);
        if (hit) {
            this._outDistance = Vector3.Distance(this.CameraFollow.transform.position, hit.point) - this.Offset;
            if (this._outDistance < this.MinDistance)
                this._outDistance = this.MinDistance;
            if (this._outDistance > this.MaxDistance)
                this._outDistance = this.MaxDistance;
        }
        else {
            this._outDistance = this.MaxDistance;
        }
        var rotation = Quaternion.FromEulerXYZ(this._pitch, this._yaw, 0);
        var targetPosition = this.CameraFollow.transform.position.Add(MathfExpand.rotateVectorByQuaternion(rotation, Vector3.back)
            .Mul(this._outDistance));
        this.transform.position = Vector3.Lerp(this.transform.position, targetPosition, this.SmoothSpeed);
        this.transform.LookAt(this.CameraFollow.transform.position);
    };
    CameraController.prototype.CameraView = function (vec) {
        this._viewHorizontal = vec.x * this.RotateSensitivity * Time.deltaTime;
        this._viewVertical = vec.y * this.RotateSensitivity * Time.deltaTime;
    };
    __decorate([
        EditorComponentSettings.DecorateName("摄像机跟随目标")
    ], CameraController.prototype, "CameraFollow", void 0);
    __decorate([
        EditorComponentSettings.DecorateName("摄像机最大距离")
    ], CameraController.prototype, "MaxDistance", void 0);
    __decorate([
        EditorComponentSettings.DecorateName("摄像机最小距离")
    ], CameraController.prototype, "MinDistance", void 0);
    __decorate([
        EditorComponentSettings.DecorateName("摄像机最大旋转角度")
    ], CameraController.prototype, "MaxRotateAngle", void 0);
    __decorate([
        EditorComponentSettings.DecorateName("摄像机最小旋转角度")
    ], CameraController.prototype, "MinRotateAngle", void 0);
    __decorate([
        EditorComponentSettings.DecorateName("摄像机旋转灵敏度")
    ], CameraController.prototype, "RotateSensitivity", void 0);
    __decorate([
        EditorComponentSettings.DecorateName("摄像机碰撞层级")
    ], CameraController.prototype, "HitLayerMask", void 0);
    __decorate([
        EditorComponentSettings.DecorateName("摄像机碰撞偏移")
    ], CameraController.prototype, "Offset", void 0);
    __decorate([
        EditorComponentSettings.DecorateName("平滑速度")
    ], CameraController.prototype, "SmoothSpeed", void 0);
    return CameraController;
}(Component));
var DecorationBase = (function (_super) {
    __extends(DecorationBase, _super);
    function DecorationBase() {
        var _this = _super.apply(this, __spreadArray([], __read(arguments), false)) || this;
        _this.ori_map = new Map();
        _this.dec_map = new Map();
        return _this;
    }
    DecorationBase.prototype.LoadDecoration = function (type) {
        var ori_go = this.ori_map.get(type);
        var asset = this.dec_map.get(type);
        if (asset == null) {
            return;
        }
        if (asset != null && ori_go) {
            ori_go.enable = false;
        }
        var prefab_go = asset.Instance();
        var tsf_list = new Array();
        DecorationStaticMeshLoader.Load(prefab_go, this, tsf_list);
        DecorationSkinLoader.Load(prefab_go, this, tsf_list);
        GameObject.DestroyGameObject(prefab_go);
    };
    DecorationBase.prototype.OnStart = function () {
        this.dec_map.set(DecorationType.Nose, this.prefab_nose);
        this.dec_map.set(DecorationType.Scarf, this.prefab_scarf);
        this.dec_map.set(DecorationType.Glasses, this.prefab_glasses);
        this.dec_map.set(DecorationType.Eyebrow, this.prefab_eyebrow);
        this.dec_map.set(DecorationType.Hat, this.prefab_hat);
        this.dec_map.set(DecorationType.Crown, this.prefab_crown);
        this.dec_map.set(DecorationType.Hair, this.prefab_hair);
        this.dec_map.set(DecorationType.Top, this.prefab_top);
        this.dec_map.set(DecorationType.Pants, this.prefab_pants);
        this.dec_map.set(DecorationType.Face, this.prefab_face);
        this.dec_map.set(DecorationType.Shoes_r, this.prefab_shoes_r);
        this.dec_map.set(DecorationType.Shoes_l, this.prefab_shoes_l);
        this.dec_map.set(DecorationType.Wing, this.prefab_wing);
        this.dec_map.set(DecorationType.Tail, this.prefab_tail);
        this.ori_map.set(DecorationType.Hair, this.gameObject.transform.FindChild("hair").gameObject);
        this.ori_map.set(DecorationType.Top, this.gameObject.transform.FindChild("trunk").gameObject);
        this.ori_map.set(DecorationType.Pants, this.gameObject.transform.FindChild("trousers").gameObject);
        this.ori_map.set(DecorationType.Face, this.gameObject.transform.FindChild("face").gameObject);
        this.ori_map.set(DecorationType.Shoes_r, this.gameObject.transform.FindChild("shoes_r").gameObject);
        this.ori_map.set(DecorationType.Shoes_l, this.gameObject.transform.FindChild("shoes_l").gameObject);
        var cnt = DecorationType.Count;
        for (var i = 0; i < cnt; i++) {
            this.LoadDecoration(i);
        }
    };
    Object.defineProperty(DecorationBase.prototype, "player", {
        get: function () {
            if (this.player_obj == null) {
                this.init();
            }
            return this.player_obj;
        },
        enumerable: false,
        configurable: true
    });
    Object.defineProperty(DecorationBase.prototype, "decorationGroup", {
        get: function () {
            if (this._dec_grp_tsf == null) {
                if (this.player_obj) {
                    var old_scene = SceneManager.activeScene;
                    SceneManager.activeScene = this.player_obj.scene;
                    var go = new GameObject("DecorationGroup");
                    SceneManager.activeScene = old_scene;
                    this._dec_grp_tsf = go.transform;
                    var player_tsf = this.player_obj.transform;
                    this._dec_grp_tsf.SetParent(player_tsf, false);
                }
            }
            return this._dec_grp_tsf;
        },
        enumerable: false,
        configurable: true
    });
    DecorationBase.prototype.GetOrGenerateSkeltonNameToTransform = function () {
        if (this.skeleton_name_to_transform == null) {
            this.init();
            this.generate_skel_name_to_transform(this.player_obj.transform);
        }
        return this.skeleton_name_to_transform;
    };
    DecorationBase.prototype.init = function () {
        if (this.player_obj) {
            return;
        }
        this.player_obj = this.gameObject;
        var skeleton_tsf = this.player_obj.transform.FindChild("Bip001");
        if (skeleton_tsf == null) {
            Debug.Error("Decoration: failed to construct decoration with null skeleton. ");
            this.reset();
            return;
        }
        this._skeleton = skeleton_tsf.gameObject;
    };
    DecorationBase.prototype.reset = function () {
        this.player_obj = null;
        this._skeleton = null;
        this.skeleton_name_to_transform = null;
    };
    DecorationBase.prototype.traverse_transform = function (root, func) {
        if (root == null) {
            return;
        }
        var cnt = root.childCount;
        func(root);
        for (var i = 0; i < cnt; i++) {
            var child = root.GetChild(i);
            this.traverse_transform(child, func);
        }
    };
    DecorationBase.prototype.generate_skel_name_to_transform = function (skeleton_tsf) {
        var _this = this;
        this.skeleton_name_to_transform = new Map();
        this.traverse_transform(skeleton_tsf, function (tsf) {
            if (tsf == null) {
                return;
            }
            _this.skeleton_name_to_transform.set(tsf.name, tsf);
        });
    };
    __decorate([
        EditorComponentSettings.DecorateName("鼻子")
    ], DecorationBase.prototype, "prefab_nose", void 0);
    __decorate([
        EditorComponentSettings.DecorateName("围脖")
    ], DecorationBase.prototype, "prefab_scarf", void 0);
    __decorate([
        EditorComponentSettings.DecorateName("眼镜")
    ], DecorationBase.prototype, "prefab_glasses", void 0);
    __decorate([
        EditorComponentSettings.DecorateName("眉毛")
    ], DecorationBase.prototype, "prefab_eyebrow", void 0);
    __decorate([
        EditorComponentSettings.DecorateName("帽子")
    ], DecorationBase.prototype, "prefab_hat", void 0);
    __decorate([
        EditorComponentSettings.DecorateName("头饰")
    ], DecorationBase.prototype, "prefab_crown", void 0);
    __decorate([
        EditorComponentSettings.DecorateName("上衣")
    ], DecorationBase.prototype, "prefab_top", void 0);
    __decorate([
        EditorComponentSettings.DecorateName("裤子")
    ], DecorationBase.prototype, "prefab_pants", void 0);
    __decorate([
        EditorComponentSettings.DecorateName("表情")
    ], DecorationBase.prototype, "prefab_face", void 0);
    __decorate([
        EditorComponentSettings.DecorateName("头发")
    ], DecorationBase.prototype, "prefab_hair", void 0);
    __decorate([
        EditorComponentSettings.DecorateName("右脚")
    ], DecorationBase.prototype, "prefab_shoes_r", void 0);
    __decorate([
        EditorComponentSettings.DecorateName("左脚")
    ], DecorationBase.prototype, "prefab_shoes_l", void 0);
    __decorate([
        EditorComponentSettings.DecorateName("翅膀")
    ], DecorationBase.prototype, "prefab_wing", void 0);
    __decorate([
        EditorComponentSettings.DecorateName("尾巴")
    ], DecorationBase.prototype, "prefab_tail", void 0);
    return DecorationBase;
}(Component));
var PlayerController = (function (_super) {
    __extends(PlayerController, _super);
    function PlayerController() {
        var _this = _super.apply(this, __spreadArray([], __read(arguments), false)) || this;
        _this.Gravity = -9.81;
        _this.MoveSpeed = 2;
        _this.JumpSpeed = 5;
        _this.JumpHeight = 1;
        _this.GroundedOffset = -1;
        _this.RotateSpeed = 10;
        _this.RotationSmoothTime = 0.1;
        _this._moveDirection = Vector3.zero;
        _this._moveHorizontal = 0;
        _this._moveVertical = 0;
        _this._groundDirection = Vector3.zero;
        _this._isJumping = false;
        _this._isGrounded = false;
        _this.CheckRadius = 0.1;
        _this.CheckDistance = 0.2;
        _this.ShowGroundEetection = false;
        return _this;
    }
    PlayerController.prototype.OnStart = function () {
        this._characterController = this.gameObject.GetComponent(CharacterController);
        this._camController = this.Cam.gameObject.GetComponent(CameraController);
        if (this._characterController == null || this.Animator == null) {
            Debug.Warning("PlayerController.OnStart: characterController or Animator is null.");
        }
    };
    PlayerController.prototype.OnUpdate = function () {
        this.JumpAndGravity();
        this.GroundedCheck();
        this.Move();
    };
    PlayerController.prototype.GroundedCheck = function () {
        var origin = this.transform.position.Add(Vector3.up.Mul(this.GroundedOffset));
        var ray = new Ray(origin, Vector3.down);
        var hit = Physics.SphereCast(ray, this.CheckRadius, this.CheckDistance, this.CheckMask);
        this._isGrounded = hit != undefined;
        if (this.Animator) {
            this.Animator.SetBool("InGround", this._isGrounded);
        }
        if (this.ShowGroundEetection) {
            Gizmos.DrawSphere(ray.origin, this.CheckRadius, true);
            Gizmos.DrawWireSphere(ray.origin.Add(Vector3.down.Mul(this.CheckDistance)), this.CheckRadius, true);
        }
    };
    PlayerController.prototype.JumpAndGravity = function () {
        if (this._isGrounded) {
            if (this._isJumping) {
                this._groundDirection.y = Mathf.Sqrt(this.JumpHeight * -2 * this.Gravity);
                if (this.Animator) {
                    this.Animator.SetTrigger("Jump");
                    this.Animator.SetBool("InJump", true);
                }
                this._isJumping = false;
            }
            else if (this._groundDirection.y < 0) {
                this._groundDirection.y = -2;
                if (this.Animator && this.Animator.GetBool("InJump") == true)
                    this.Animator.SetBool("InJump", false);
            }
        }
        else {
            this._groundDirection.y += this.Gravity * this.JumpSpeed * Time.deltaTime;
        }
        this._characterController.Move(this._groundDirection.Mul(Time.deltaTime));
    };
    PlayerController.prototype.Move = function () {
        var moveDir = new Vector3(this._moveHorizontal, 0, this._moveVertical);
        this._moveDirection = moveDir.magnitude != 0 ? moveDir.normalized : Vector3.zero;
        if (this._moveDirection.magnitude >= 0.1) {
            var targetAngle = Mathf.ATan2(this._moveDirection.x, this._moveDirection.z) * Mathf.radToAngle + this.Cam.transform.eulerAngles.y;
            var currentQuaternion = this.transform.rotation;
            var targetQuaternion = Quaternion.FromEulerXYZ(0, targetAngle, 0);
            var t = Time.deltaTime * this.RotateSpeed;
            var interpolatedQuaternion = Quaternion.Slerp(currentQuaternion, targetQuaternion, t);
            this.transform.rotation = interpolatedQuaternion;
            var moveDir_1 = MathfExpand.rotateVectorByQuaternion(interpolatedQuaternion, Vector3.forward);
            this._characterController.Move(moveDir_1.normalized.Mul(this.MoveSpeed * Time.deltaTime));
            if (this.Animator)
                this.Animator.SetFloat("Move", this._moveDirection.magnitude);
        }
        else {
            if (this.Animator)
                this.Animator.SetFloat("Move", 0);
        }
    };
    PlayerController.prototype.CharacterMove = function (vec) {
        this._moveHorizontal = vec.x;
        this._moveVertical = vec.y;
    };
    PlayerController.prototype.CharacterView = function (vec) {
        this._camController.CameraView(vec);
    };
    PlayerController.prototype.CharacterJump = function () {
        if (this._isGrounded)
            this._isJumping = true;
    };
    __decorate([
        EditorComponentSettings.DecorateName("角色重力")
    ], PlayerController.prototype, "Gravity", void 0);
    __decorate([
        EditorComponentSettings.DecorateName("角色动画控制器")
    ], PlayerController.prototype, "Animator", void 0);
    __decorate([
        EditorComponentSettings.DecorateName("角色摄像机")
    ], PlayerController.prototype, "Cam", void 0);
    __decorate([
        EditorComponentSettings.DecorateName("角色控制器")
    ], PlayerController.prototype, "_characterController", void 0);
    __decorate([
        EditorComponentSettings.DecorateName("摄像机控制器")
    ], PlayerController.prototype, "_camController", void 0);
    __decorate([
        EditorComponentSettings.DecorateName("角色移动速度")
    ], PlayerController.prototype, "MoveSpeed", void 0);
    __decorate([
        EditorComponentSettings.DecorateName("角色跳跃速度")
    ], PlayerController.prototype, "JumpSpeed", void 0);
    __decorate([
        EditorComponentSettings.DecorateName("角色跳跃高度")
    ], PlayerController.prototype, "JumpHeight", void 0);
    __decorate([
        EditorComponentSettings.DecorateName("角色地面偏移")
    ], PlayerController.prototype, "GroundedOffset", void 0);
    __decorate([
        EditorComponentSettings.DecorateName("角色转身速度")
    ], PlayerController.prototype, "RotateSpeed", void 0);
    __decorate([
        EditorComponentSettings.DecorateName("角色转身平滑时间")
    ], PlayerController.prototype, "RotationSmoothTime", void 0);
    __decorate([
        EditorComponentSettings.DecorateName("角色转身平滑速度")
    ], PlayerController.prototype, "_moveDirection", void 0);
    __decorate([
        EditorComponentSettings.DecorateName("角色转身平滑速度")
    ], PlayerController.prototype, "_moveHorizontal", void 0);
    __decorate([
        EditorComponentSettings.DecorateName("角色转身平滑速度")
    ], PlayerController.prototype, "_moveVertical", void 0);
    __decorate([
        EditorComponentSettings.DecorateName("角色转身平滑速度")
    ], PlayerController.prototype, "_groundDirection", void 0);
    __decorate([
        EditorComponentSettings.DecorateName("检测半径")
    ], PlayerController.prototype, "CheckRadius", void 0);
    __decorate([
        EditorComponentSettings.DecorateName("检测距离")
    ], PlayerController.prototype, "CheckDistance", void 0);
    __decorate([
        EditorComponentSettings.DecorateName("检测层级"),
        EditorComponentSettings.Tooltip("二进制数字")
    ], PlayerController.prototype, "CheckMask", void 0);
    __decorate([
        EditorComponentSettings.DecorateName("显示检测球体")
    ], PlayerController.prototype, "ShowGroundEetection", void 0);
    return PlayerController;
}(Component));
var __values = (this && this.__values) || function(o) {
    var s = typeof Symbol === "function" && Symbol.iterator, m = s && o[s], i = 0;
    if (m) return m.call(o);
    if (o && typeof o.length === "number") return {
        next: function () {
            if (o && i >= o.length) o = void 0;
            return { value: o && o[i++], done: !o };
        }
    };
    throw new TypeError(s ? "Object is not iterable." : "Symbol.iterator is not defined.");
};
var DecorationSkinLoader = (function () {
    function DecorationSkinLoader() {
    }
    DecorationSkinLoader.Load = function (prefab_obj, dec_ctrl, list) {
        var e_1, _a;
        if (prefab_obj == null || dec_ctrl == null) {
            return;
        }
        var player_obj = dec_ctrl.player;
        if (player_obj == null) {
            return;
        }
        var smr_list = prefab_obj.GetComponentsInChildren(SkinnedMeshRenderer);
        if (smr_list == null || smr_list.length < 1) {
            return;
        }
        var skel_name_to_tsf = dec_ctrl.GetOrGenerateSkeltonNameToTransform();
        if (skel_name_to_tsf == null) {
            Debug.Error("DecorationSkinLoader: invalid skeleton.");
            return;
        }
        var dec_grp_tsf = dec_ctrl.decorationGroup;
        try {
            for (var smr_list_1 = __values(smr_list), smr_list_1_1 = smr_list_1.next(); !smr_list_1_1.done; smr_list_1_1 = smr_list_1.next()) {
                var smr = smr_list_1_1.value;
                var part_bone_arr = smr.bones;
                var new_bone_arr = new Array();
                for (var i = 0; i < part_bone_arr.length; i++) {
                    var new_bone = skel_name_to_tsf.get(part_bone_arr[i].name);
                    if (new_bone == null) {
                        Debug.Error("can't find bone ", part_bone_arr[i].name);
                    }
                    new_bone_arr.push(new_bone);
                }
                smr.bones = new_bone_arr;
                if (smr.rootBone) {
                    smr.rootBone = skel_name_to_tsf.get(smr.rootBone.name);
                }
                var mtl = smr.material;
                if (mtl.shader == null) {
                    mtl.shader = Shader.Find("Engine/Default");
                }
                var smr_go = smr.gameObject;
                var smr_tsf = smr_go.transform;
                smr_tsf.SetParent(dec_grp_tsf, false);
                list.push(smr_tsf);
            }
        }
        catch (e_1_1) { e_1 = { error: e_1_1 }; }
        finally {
            try {
                if (smr_list_1_1 && !smr_list_1_1.done && (_a = smr_list_1.return)) _a.call(smr_list_1);
            }
            finally { if (e_1) throw e_1.error; }
        }
        return;
    };
    return DecorationSkinLoader;
}());
var DecorationStaticMeshLoader = (function () {
    function DecorationStaticMeshLoader() {
    }
    DecorationStaticMeshLoader.Load = function (prefab_obj, dec_ctrl, list) {
        var e_2, _a;
        if (list == null) {
            return;
        }
        if (prefab_obj == null || dec_ctrl == null) {
            return;
        }
        var skel_name_to_tsf = dec_ctrl.GetOrGenerateSkeltonNameToTransform();
        if (skel_name_to_tsf == null) {
            Debug.Error("DecorationStaticMeshLoader: invalid skeleton.");
            return null;
        }
        var renderer_list = prefab_obj.GetComponentsInChildren(MeshRenderer);
        if (renderer_list == null || renderer_list.length < 1) {
            return;
        }
        var prefab_obj_name = prefab_obj.name;
        try {
            for (var renderer_list_1 = __values(renderer_list), renderer_list_1_1 = renderer_list_1.next(); !renderer_list_1_1.done; renderer_list_1_1 = renderer_list_1.next()) {
                var renderer = renderer_list_1_1.value;
                var renderer_obj = renderer.gameObject;
                var mount_pt_name = renderer_obj.name;
                var mtl = renderer.material;
                if (mtl.shader == null) {
                    mtl.shader = Shader.Find("Engine/Default");
                }
                var mount_pt_tsf = null;
                if (skel_name_to_tsf.has(mount_pt_name) == false) {
                    mount_pt_tsf = dec_ctrl.player.transform;
                }
                else {
                    mount_pt_tsf = skel_name_to_tsf.get(mount_pt_name);
                }
                renderer_obj.name = prefab_obj_name + "_" + mount_pt_name;
                var render_tsf = renderer_obj.transform;
                render_tsf.SetParent(mount_pt_tsf, false);
                list.push(render_tsf);
            }
        }
        catch (e_2_1) { e_2 = { error: e_2_1 }; }
        finally {
            try {
                if (renderer_list_1_1 && !renderer_list_1_1.done && (_a = renderer_list_1.return)) _a.call(renderer_list_1);
            }
            finally { if (e_2) throw e_2.error; }
        }
    };
    return DecorationStaticMeshLoader;
}());
var JoyStick = (function (_super) {
    __extends(JoyStick, _super);
    function JoyStick() {
        var _this = _super.apply(this, __spreadArray([], __read(arguments), false)) || this;
        _this.IsJoyStickInputEnable = true;
        _this.ScreenMoveDeadZone = 5;
        return _this;
    }
    JoyStick.prototype.OnStart = function () {
        var _this = this;
        this._uiComponent = this.gameObject.GetComponent(UIComponent);
        var canvas = this._uiComponent.canvas;
        this._leftJoyStick = canvas.FindChild(Image, this.LeftJoyStick);
        this._leftJoyStickCap = this._leftJoyStick.FindChild(Image, this.LeftJoyStickCap);
        this._jumpButton = canvas.FindChild(Image, this.JumpButton);
        this._screenMoveArea = canvas.FindChild(Image, this.ScreenMoveArea);
        this._leftJoyStick.AddEvent(ControlEvent.EventPointerDrag, function (control) {
            _this.ControlTarget.CharacterMove(_this.PlayerMove(_this._leftJoyStick, _this._leftJoyStickCap));
        });
        this._leftJoyStick.AddEvent(ControlEvent.EventPointerStationary, function (control) {
            var myEventData = _this._leftJoyStick.guiEventData;
            _this.ControlTarget.CharacterMove(_this.PlayerMove(_this._leftJoyStick, _this._leftJoyStickCap));
        });
        this._leftJoyStick.AddEvent(ControlEvent.EventPointerUp, function (control) {
            var thumb = _this._leftJoyStick.FindChild(Image, _this.LeftJoyStickCap);
            var thumbRectTransform = thumb.rectTransform;
            thumbRectTransform.localPosition = Vector2.zero;
            _this.ControlTarget.CharacterMove(Vector2.zero);
        });
        this._screenMoveArea.AddEvent(ControlEvent.EventPointerDrag, function (control) {
            var myEventData = _this._screenMoveArea.guiEventData;
            var dragPosition = myEventData.moveDelta;
            dragPosition.y = -dragPosition.y;
            _this.ControlTarget.CharacterView(dragPosition);
        });
        this._screenMoveArea.AddEvent(ControlEvent.EventPointerStationary, function (control) {
            _this.ControlTarget.CharacterView(Vector2.zero);
        });
        this._screenMoveArea.AddEvent(ControlEvent.EventPointerUp, function (control) {
            _this.ControlTarget.CharacterView(Vector2.zero);
        });
        this._jumpButton.AddEvent(ControlEvent.EventPointerDown, function (control) {
            _this.ControlTarget.CharacterJump();
        });
    };
    JoyStick.prototype.PlayerMove = function (leftJoyStick, leftJoyStickCap) {
        var myEventData = leftJoyStick.guiEventData;
        var thumbRectTransform = leftJoyStickCap.rectTransform;
        var joystickRectTransform = leftJoyStick.rectTransform;
        var dragPosition = myEventData.position;
        var radius = joystickRectTransform.width / 2;
        var oriPos = joystickRectTransform.worldPosition.Add(new Vector2(radius, -radius));
        var offset = dragPosition.Sub(oriPos);
        var thumbPos = new Vector2();
        if (offset.magnitude > radius) {
            thumbPos = offset.normalized.Mul(radius);
        }
        else {
            thumbPos = offset;
        }
        thumbRectTransform.localPosition = thumbPos;
        var move = new Vector2(offset.x, -offset.y).normalized;
        return move;
    };
    __decorate([
        EditorComponentSettings.DecorateName("是否启用摇杆输入")
    ], JoyStick.prototype, "IsJoyStickInputEnable", void 0);
    __decorate([
        EditorComponentSettings.DecorateName("控制目标")
    ], JoyStick.prototype, "ControlTarget", void 0);
    __decorate([
        EditorComponentSettings.DecorateName("左摇杆")
    ], JoyStick.prototype, "LeftJoyStick", void 0);
    __decorate([
        EditorComponentSettings.DecorateName("左摇杆盖")
    ], JoyStick.prototype, "LeftJoyStickCap", void 0);
    __decorate([
        EditorComponentSettings.DecorateName("屏幕移动区域")
    ], JoyStick.prototype, "ScreenMoveArea", void 0);
    __decorate([
        EditorComponentSettings.DecorateName("屏幕移动死区")
    ], JoyStick.prototype, "ScreenMoveDeadZone", void 0);
    __decorate([
        EditorComponentSettings.DecorateName("跳跃按钮")
    ], JoyStick.prototype, "JumpButton", void 0);
    return JoyStick;
}(Component));
var KeyBoard = (function (_super) {
    __extends(KeyBoard, _super);
    function KeyBoard() {
        var _this = _super.apply(this, __spreadArray([], __read(arguments), false)) || this;
        _this.IsKeyBoardInputEnable = true;
        _this.ArrowKeyForward = KeyCode.W;
        _this.ArrowKeyBack = KeyCode.S;
        _this.ArrowKeyLeft = KeyCode.A;
        _this.ArrowKeyRight = KeyCode.D;
        return _this;
    }
    KeyBoard.prototype.OnEnable = function () {
        if (!this.IsKeyBoardInputEnable) {
            this.enable = false;
        }
    };
    KeyBoard.prototype.OnUpdate = function () {
        this.PlayerMove();
        this.PlayerJump();
    };
    KeyBoard.prototype.PlayerMove = function () {
        var vertical = Input.GetKey(KeyCode.W) ? 1 : (Input.GetKey(KeyCode.S) ? -1 : 0);
        var horizontal = Input.GetKey(KeyCode.D) ? 1 : (Input.GetKey(KeyCode.A) ? -1 : 0);
        var distance = new Vector2(horizontal, vertical);
        if (!distance.EqualsTo(Vector2.zero))
            distance = distance.normalized;
        this.ControlTarget.CharacterMove(distance);
    };
    KeyBoard.prototype.PlayerJump = function () {
        if (Input.GetKeyDown(KeyCode.Space))
            this.ControlTarget.CharacterJump();
    };
    __decorate([
        EditorComponentSettings.DecorateName("是否启用键盘输入")
    ], KeyBoard.prototype, "IsKeyBoardInputEnable", void 0);
    __decorate([
        EditorComponentSettings.DecorateName("控制目标")
    ], KeyBoard.prototype, "ControlTarget", void 0);
    return KeyBoard;
}(Component));
var MathfExpand = (function () {
    function MathfExpand() {
    }
    MathfExpand.rotateVectorByQuaternion = function (q, v) {
        var vectorAsQuaternion = new Quaternion(v.x, v.y, v.z, 0);
        var result = q.Mul(vectorAsQuaternion).Mul(q.inverse);
        return new Vector3(result.x, result.y, result.z);
    };
    return MathfExpand;
}());
var DecorationType;
(function (DecorationType) {
    DecorationType[DecorationType["Nose"] = 0] = "Nose";
    DecorationType[DecorationType["Scarf"] = 1] = "Scarf";
    DecorationType[DecorationType["Glasses"] = 2] = "Glasses";
    DecorationType[DecorationType["Eyebrow"] = 3] = "Eyebrow";
    DecorationType[DecorationType["Hat"] = 4] = "Hat";
    DecorationType[DecorationType["Crown"] = 5] = "Crown";
    DecorationType[DecorationType["Hair"] = 6] = "Hair";
    DecorationType[DecorationType["Face"] = 7] = "Face";
    DecorationType[DecorationType["Top"] = 8] = "Top";
    DecorationType[DecorationType["Pants"] = 9] = "Pants";
    DecorationType[DecorationType["Shoes_r"] = 10] = "Shoes_r";
    DecorationType[DecorationType["Shoes_l"] = 11] = "Shoes_l";
    DecorationType[DecorationType["Wing"] = 12] = "Wing";
    DecorationType[DecorationType["Tail"] = 13] = "Tail";
    DecorationType[DecorationType["Count"] = 14] = "Count";
})(DecorationType || (DecorationType = {}));
//# sourceMappingURL=data:application/json;base64,eyJ2ZXJzaW9uIjozLCJmaWxlIjoiQXNzZW1ibHkuanMiLCJzb3VyY2VSb290IjoiIiwic291cmNlcyI6WyIuLi8uLi9Bc3NldHMvU2NyaXB0cy9Db250cm9sbGVyL0NhbWVyYUNvbnRyb2xsZXIudHMiLCIuLi8uLi9Bc3NldHMvU2NyaXB0cy9EZWNvcmF0aW9uL0RlY29yYXRpb25CYXNlLnRzIiwiLi4vLi4vQXNzZXRzL1NjcmlwdHMvQ29udHJvbGxlci9QbGF5ZXJDb250cm9sbGVyLnRzIiwiLi4vLi4vQXNzZXRzL1NjcmlwdHMvRGVjb3JhdGlvbi9EZWNvcmF0aW9uU2tpbkxvYWRlci50cyIsIi4uLy4uL0Fzc2V0cy9TY3JpcHRzL0RlY29yYXRpb24vRGVjb3JhdGlvblN0YXRpY01lc2hMb2FkZXIudHMiLCIuLi8uLi9Bc3NldHMvU2NyaXB0cy9JbnB1dENvbnRyb2xsZXIvSm95U3RpY2sudHMiLCIuLi8uLi9Bc3NldHMvU2NyaXB0cy9JbnB1dENvbnRyb2xsZXIvS2V5Qm9hcmQudHMiLCIuLi8uLi9Bc3NldHMvU2NyaXB0cy9Vbml0cy9NYXRoZkV4cGFuZC50cyIsIi4uLy4uL0Fzc2V0cy9TY3JpcHRzL0RlY29yYXRpb24vRGVjb3JhdGlvblR5cGUudHMiXSwibmFtZXMiOltdLCJtYXBwaW5ncyI6Ijs7Ozs7Ozs7Ozs7Ozs7Ozs7Ozs7Ozs7Ozs7Ozs7Ozs7Ozs7Ozs7Ozs7Ozs7OztBQUFBO0lBQStCLG9DQUFTO0lBQXhDOztRQUtRLGlCQUFXLEdBQVcsQ0FBQyxDQUFDO1FBRXhCLGlCQUFXLEdBQVcsQ0FBQyxDQUFDO1FBRXhCLG9CQUFjLEdBQVcsRUFBRSxDQUFDO1FBRTVCLG9CQUFjLEdBQVcsQ0FBQyxFQUFFLENBQUM7UUFFN0IsdUJBQWlCLEdBQVcsRUFBRSxDQUFDO1FBSS9CLFlBQU0sR0FBVyxHQUFHLENBQUM7UUFFckIsaUJBQVcsR0FBVyxHQUFHLENBQUM7UUFFekIscUJBQWUsR0FBVyxDQUFDLENBQUM7UUFDNUIsbUJBQWEsR0FBVyxDQUFDLENBQUM7UUFDMUIsVUFBSSxHQUFXLENB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QztRQUN0RCxJQUFJLENBQUMsT0FBTyxDQUFDLEdBQUcsQ0FBQyxjQUFjLENBQUMsS0FBSyxFQUFFLElBQUksQ0FBQyxZQUFZLENBQUMsQ0FBQztRQUMxRCxJQUFJLENBQUMsT0FBTyxDQUFDLEdBQUcsQ0FBQyxjQUFjLENBQUMsSUFBSSxFQUFFLElBQUksQ0FBQyxXQUFXLENBQUMsQ0FBQztRQUN4RCxJQUFJLENBQUMsT0FBTyxDQUFDLEdBQUcsQ0FBQyxjQUFjLENBQUMsT0FBTyxFQUFFLElBQUksQ0FBQyxjQUFjLENBQUMsQ0FBQztRQUM5RCxJQUFJLENBQUMsT0FBTyxDQUFDLEdBQUcsQ0FBQyxjQUFjLENBQUMsT0FBTyxFQUFFLElBQUksQ0FBQyxjQUFjLENBQUMsQ0FBQztRQUM5RCxJQUFJLENBQUMsT0FBTyxDQUFDLEdBQUcsQ0FBQyxjQUFjLENBQUMsSUFBSSxFQUFFLElBQUksQ0FBQyxXQUFXLENBQUMsQ0FBQztRQUN4RCxJQUFJLENBQUMsT0FBTyxDQUFDLEdBQUcsQ0FBQyxjQUFjLENBQUMsSUFBSSxFQUFFLElBQUksQ0FBQyxXQUFXLENBQUMsQ0FBQztRQUd4RCxJQUFJLENBQUMsT0FBTyxDQUFDLEdBQUcsQ0FBQyxjQUFjLENBQUMsSUFBSSxFQUFFLElBQUksQ0FBQyxVQUFVLENBQUMsU0FBUyxDQUFDLFNBQVMsQ0FBQyxNQUFNLENBQUMsQ0FBQyxVQUFVLENBQUMsQ0FBQztRQUM5RixJQUFJLENBQUMsT0FBTyxDQUFDLEdBQUcsQ0FBQyxjQUFjLENBQUMsR0FBRyxFQUFFLElBQUksQ0FBQyxVQUFVLENBQUMsU0FBUyxDQUFDLFNBQVMsQ0FBQyxPQUFPLENBQUMsQ0FBQyxVQUFVLENBQUMsQ0FBQztRQUM5RixJQUFJLENBQUMsT0FBTyxDQUFDLEdBQUcsQ0FBQyxjQUFjLENBQUMsS0FBSyxFQUFFLElBQUksQ0FBQyxVQUFVLENBQUMsU0FBUyxDQUFDLFNBQVMsQ0FBQyxVQUFVLENBQUMsQ0FBQyxVQUFVLENBQUMsQ0FBQztRQUNuRyxJQUFJLENBQUMsT0FBTyxDQUFDLEdBQUcsQ0FB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