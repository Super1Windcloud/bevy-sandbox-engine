class DecorationSkinLoader {
    static Load(prefab_obj: GameObject, dec_ctrl: DecorationBase, list: Array<Transform>): Array<Transform>{
        if (prefab_obj == null || dec_ctrl == null) { return; }
        let player_obj = dec_ctrl.player;
        if (player_obj == null) { return; }

        // 获取当前预制体的所有SkinnedMeshRenderer
        let smr_list = prefab_obj.GetComponentsInChildren(SkinnedMeshRenderer);
        if (smr_list == null || smr_list.length < 1) { 
            return; 
        }

        let skel_name_to_tsf = dec_ctrl.GetOrGenerateSkeltonNameToTransform();
        if (skel_name_to_tsf == null) {
            Debug.Error("DecorationSkinLoader: invalid skeleton.");
            return;
        }

        // 遍历所有SkinnedMeshRenderer，将骨骼绑定到对应的骨骼上
        let dec_grp_tsf = dec_ctrl.decorationGroup;
        for (let smr of smr_list){
            let part_bone_arr = smr.bones;
            let new_bone_arr = new Array<Transform>();
            for (let i = 0; i < part_bone_arr.length; i++){
                let new_bone = skel_name_to_tsf.get(part_bone_arr[i].name);
                if (new_bone == null) {
                    Debug.Error("can't find bone ", part_bone_arr[i].name);
                }
                new_bone_arr.push(new_bone);
            }

            smr.bones = new_bone_arr;
            if (smr.rootBone){
                smr.rootBone = skel_name_to_tsf.get(smr.rootBone.name);
            }

            let mtl = smr.material;
            if (mtl.shader == null){
                mtl.shader = Shader.Find("Engine/Default");
            }

            // 将SkinnedMeshRenderer添加到装饰组中
            let smr_go = smr.gameObject;
            let smr_tsf = smr_go.transform;
            smr_tsf.SetParent(dec_grp_tsf,false);
            list.push(smr_tsf);
        }

        return;
    }
}