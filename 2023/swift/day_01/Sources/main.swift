// The Swift Programming Language
// https://docs.swift.org/swift-book
import Foundation

let filePath = "../../inputs/day_01.txt"

do {
	let fileContent = try String(contentsOfFile: filePath)
	for line in fileContent.components(separatedBy: "\n") {
		print(line)
	}	// print(fileContent)
}
catch {
	print("Error reading file: \(error)")
}
