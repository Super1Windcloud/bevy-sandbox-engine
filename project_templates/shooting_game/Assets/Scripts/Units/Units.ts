/**
 * 工具类，集成一些常用工具方法
 * 所有方法都应为静态方法
 */
class Units
{
	/**
	 * 用于检查类型的子类型关系，如果类型不匹配则会抛出异常
	 * 用于通过父类来判断子类，并调用子类方法
	 * @param component 目标类型
	 * @returns 
	 */
	public static AssertSubtype<BaseTytpe, ChildType extends BaseTytpe>(component: ChildType): ChildType
	{
		return component;
	}

	/**
	 * 检查指定对象是否为空，如果为空则打印警告信息
	 * 可以提供指定对象所在的GameObject，以便打印更多信息
	 * @param obj 指定对象
	 * @param msg 警告信息
	 * @param go 指定对象所在的GameObject
	 */
	public static CheckNull(obj: any, msg: string, go?:GameObject): void
	{
		if (obj == null)
		{
			if (go != undefined)
				Debug.Warning(go.name + ": " + msg);
			else
				Debug.Warning(msg);
		}
	}
}